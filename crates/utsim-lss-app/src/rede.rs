//! The gemba page (P7, "acesso pela rede local com token"; LSS-RNF-07): the open actions of
//! the project as a read-only web page, for a tablet or phone on the shop floor.
//!
//! - Off by default; turned on in the team screen.
//! - Listens on `127.0.0.1` unless the local network is opened explicitly. On `127.0.0.1` only
//!   requests whose `Host` is this computer are served (no page of another site reads it
//!   through DNS rebinding).
//! - Opened to the network, every request must carry the access token (`/?t=…`), 128 random
//!   bits made when the page is turned on; without it the answer is 403.
//! - Read only: `GET` and `HEAD` of `/`. Nothing is edited through the page, so the journal,
//!   the signatures and the profiles stay the window's business.
//!
//! One thread accepts, one short thread per connection (at most [`MAX_CONNECTIONS`]). The
//! window rebuilds the page from the state every few seconds and hands it over with
//! [`Gemba::set_page`].

use std::io::{self, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use utsim_lss_core::date::Date;
use utsim_lss_core::domain::{self, Project};
use utsim_lss_core::id::Id;
use utsim_lss_core::kanban;
use utsim_lss_core::state::State;

use crate::text::l13::l13;
use crate::text::l15::l15;
use crate::text::{self, LocalDate};

/// Connections served at the same time; more wait in the queue of the system.
pub const MAX_CONNECTIONS: usize = 16;
/// The largest request read (the page asks for nothing larger).
const MAX_REQUEST: usize = 8 * 1024;
/// How often the window rebuilds the page.
pub const REFRESH: Duration = Duration::from_secs(3);

struct Shared {
    page: Mutex<Arc<Vec<u8>>>,
    stop: AtomicBool,
    open: AtomicUsize,
    /// The 403 page, in the window's language when the page was turned on.
    forbidden: Arc<Vec<u8>>,
}

/// The page turned on.
pub struct Gemba {
    shared: Arc<Shared>,
    addr: SocketAddr,
    token: Option<String>,
    url: String,
    accept: Option<JoinHandle<()>>,
    built: Option<Instant>,
}

impl Gemba {
    /// Turns the page on at `port` (0: any free port). `lan` opens it to the local network,
    /// with a token.
    pub fn start(lan: bool, port: u16) -> io::Result<Gemba> {
        let ip = if lan {
            Ipv4Addr::UNSPECIFIED
        } else {
            Ipv4Addr::LOCALHOST
        };
        Gemba::start_on(ip, lan, port)
    }

    /// Turns the page on at an address of this computer; `with_token` asks the token as when
    /// the page is open to the network (the tests use it on `127.0.0.1`, which opens no door
    /// in the firewall).
    pub fn start_on(ip: Ipv4Addr, with_token: bool, port: u16) -> io::Result<Gemba> {
        let lan = with_token;
        let listener = TcpListener::bind((ip, port))?;
        listener.set_nonblocking(true)?;
        let addr = listener.local_addr()?;
        let token = if lan { Some(new_token()?) } else { None };
        let shared = Arc::new(Shared {
            page: Mutex::new(Arc::new(Vec::new())),
            stop: AtomicBool::new(false),
            open: AtomicUsize::new(0),
            forbidden: Arc::new(error_page(l15().forbidden).into_bytes()),
        });
        let sh = shared.clone();
        let tk = token.clone();
        let accept = thread::Builder::new()
            .name("gemba".into())
            .spawn(move || accept_loop(listener, sh, tk, addr.port()))?;
        let url = match &token {
            Some(t) => {
                let host = network_address()
                    .map(|a| a.to_string())
                    .unwrap_or_else(hostname);
                format!("http://{host}:{}/?t={t}", addr.port())
            }
            None => format!("http://localhost:{}/", addr.port()),
        };
        Ok(Gemba {
            shared,
            addr,
            token,
            url,
            accept: Some(accept),
            built: None,
        })
    }

    pub fn port(&self) -> u16 {
        self.addr.port()
    }

    /// Whether the page is open to the local network.
    pub fn lan(&self) -> bool {
        self.token.is_some()
    }

    /// The address to type on the tablet: this computer's address on the network and the
    /// token, or `localhost` when the page is only for this computer.
    pub fn url(&self) -> String {
        self.url.clone()
    }

    /// Whether the page is due to be rebuilt (and marks it as rebuilt now).
    pub fn due(&mut self) -> bool {
        let due = self.built.is_none_or(|t| t.elapsed() >= REFRESH);
        if due {
            self.built = Some(Instant::now());
        }
        due
    }

    /// The page served from now on.
    pub fn set_page(&self, html: String) {
        *self.shared.page.lock().unwrap_or_else(|e| e.into_inner()) = Arc::new(html.into_bytes());
    }
}

impl Drop for Gemba {
    fn drop(&mut self) {
        self.shared.stop.store(true, Ordering::SeqCst);
        if let Some(h) = self.accept.take() {
            let _ = h.join();
        }
    }
}

/// 128 random bits in hexadecimal.
fn new_token() -> io::Result<String> {
    let mut b = [0u8; 16];
    getrandom::getrandom(&mut b).map_err(io::Error::other)?;
    Ok(b.iter().map(|x| format!("{x:02x}")).collect())
}

/// The address of this computer on the local network: the one the system would use to
/// reach a private address (a UDP "connect" sends nothing).
fn network_address() -> Option<std::net::IpAddr> {
    let s = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).ok()?;
    s.connect((Ipv4Addr::new(10, 254, 254, 254), 9)).ok()?;
    let ip = s.local_addr().ok()?.ip();
    (!ip.is_unspecified() && !ip.is_loopback()).then_some(ip)
}

fn hostname() -> String {
    std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_else(|_| "localhost".into())
}

fn accept_loop(listener: TcpListener, shared: Arc<Shared>, token: Option<String>, port: u16) {
    while !shared.stop.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((stream, _)) => {
                if shared.open.load(Ordering::SeqCst) >= MAX_CONNECTIONS {
                    let _ = answer(stream, "503 Service Unavailable", b"", false);
                    continue;
                }
                shared.open.fetch_add(1, Ordering::SeqCst);
                let sh = shared.clone();
                let tk = token.clone();
                let spawned = thread::Builder::new().name("gemba-conn".into()).spawn({
                    let sh = sh.clone();
                    move || {
                        let _ = serve(stream, &sh, tk.as_deref(), port);
                        sh.open.fetch_sub(1, Ordering::SeqCst);
                    }
                });
                if spawned.is_err() {
                    sh.open.fetch_sub(1, Ordering::SeqCst);
                }
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(50));
            }
            Err(_) => thread::sleep(Duration::from_millis(200)),
        }
    }
}

/// A request line and the `Host` header.
struct Request {
    method: String,
    target: String,
    host: String,
}

fn read_request(stream: &mut TcpStream) -> io::Result<Option<Request>> {
    stream.set_nonblocking(false)?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;
    let mut buf = Vec::new();
    let mut chunk = [0u8; 1024];
    while !buf.windows(4).any(|w| w == b"\r\n\r\n") {
        if buf.len() > MAX_REQUEST {
            return Ok(None);
        }
        let n = stream.read(&mut chunk)?;
        if n == 0 {
            return Ok(None);
        }
        buf.extend_from_slice(&chunk[..n]);
    }
    let text = String::from_utf8_lossy(&buf);
    let mut lines = text.split("\r\n");
    let mut first = lines.next().unwrap_or("").split(' ');
    let method = first.next().unwrap_or("").to_string();
    let target = first.next().unwrap_or("").to_string();
    let host = lines
        .take_while(|l| !l.is_empty())
        .find_map(|l| {
            let (k, v) = l.split_once(':')?;
            k.trim()
                .eq_ignore_ascii_case("host")
                .then(|| v.trim().to_string())
        })
        .unwrap_or_default();
    Ok(Some(Request {
        method,
        target,
        host,
    }))
}

fn serve(mut stream: TcpStream, shared: &Shared, token: Option<&str>, port: u16) -> io::Result<()> {
    let Some(req) = read_request(&mut stream)? else {
        return answer(stream, "400 Bad Request", b"", false);
    };
    let head = req.method == "HEAD";
    let (path, query) = req.target.split_once('?').unwrap_or((&req.target, ""));
    let (status, body): (&str, Arc<Vec<u8>>) = if req.method != "GET" && !head {
        ("405 Method Not Allowed", Arc::default())
    } else if !allowed(token, &req.host, query, port) {
        ("403 Forbidden", shared.forbidden.clone())
    } else if path != "/" {
        ("404 Not Found", Arc::default())
    } else {
        let page = shared
            .page
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone();
        ("200 OK", page)
    };
    let body: &[u8] = if head { b"" } else { &body };
    answer(stream, status, body, true)
}

/// Whether a request may read the page: with a token, the query carries it; without, the
/// host is this computer.
fn allowed(token: Option<&str>, host: &str, query: &str, port: u16) -> bool {
    match token {
        Some(t) => query
            .split('&')
            .filter_map(|kv| kv.strip_prefix("t="))
            .any(|v| same(v.as_bytes(), t.as_bytes())),
        None => host_is_local(host, port),
    }
}

/// Comparison that takes the same time wherever the first difference is.
fn same(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).fold(0u8, |d, (x, y)| d | (x ^ y)) == 0
}

fn host_is_local(host: &str, port: u16) -> bool {
    let p = port.to_string();
    ["localhost", "127.0.0.1", "[::1]"]
        .iter()
        .any(|h| host == *h || host.strip_prefix(h).and_then(|r| r.strip_prefix(':')) == Some(&p))
}

fn answer(mut stream: TcpStream, status: &str, body: &[u8], html: bool) -> io::Result<()> {
    let kind = if html {
        "text/html; charset=utf-8"
    } else {
        "text/plain; charset=utf-8"
    };
    let head = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {kind}\r\nContent-Length: {}\r\n\
         Cache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\n\
         Content-Security-Policy: default-src 'none'; style-src 'unsafe-inline'\r\n\
         Referrer-Policy: no-referrer\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(head.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()
}

/// Text put in HTML.
fn esc(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => o.push_str("&amp;"),
            '<' => o.push_str("&lt;"),
            '>' => o.push_str("&gt;"),
            '"' => o.push_str("&quot;"),
            '\'' => o.push_str("&#39;"),
            c => o.push(c),
        }
    }
    o
}

const STYLE: &str = "body{font-family:system-ui,sans-serif;margin:16px;color:#1d2433;background:#f6f7f9}\
h1{font-size:1.4em;margin:0 0 4px}h2{font-size:1.1em;margin:20px 0 6px}\
p.n{color:#5b6475;margin:0 0 12px}table{border-collapse:collapse;width:100%;background:#fff}\
th,td{text-align:left;padding:8px;border-bottom:1px solid #dde1e8;vertical-align:top}\
th{background:#eef1f5}.late{color:#b3261e;font-weight:600}\
@media (prefers-color-scheme:dark){body{background:#15181e;color:#e4e7ec}\
table{background:#1d2129}th{background:#262b35}th,td{border-color:#2f3541}\
p.n{color:#9aa3b2}.late{color:#ff8a80}}";

fn error_page(text: &str) -> String {
    format!(
        "<!doctype html><html><head><meta charset=\"utf-8\">\
         <meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\
         <style>{STYLE}</style></head><body><p>{}</p></body></html>",
        esc(text)
    )
}

/// The page: the open actions of the project, by person (who has none last), each by due
/// date. `updated` is the time shown as the moment of the page.
pub fn page(state: &State, project: Id, today: Date, updated: &str) -> String {
    let t = l15();
    let tc = l13();
    let title = state
        .entity(project)
        .map(|e| Project::read(project, e).title)
        .unwrap_or_default();
    let mut h = format!(
        "<!doctype html><html lang=\"{}\"><head><meta charset=\"utf-8\">\
         <meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\
         <meta http-equiv=\"refresh\" content=\"30\"><title>{}</title><style>{STYLE}</style>\
         </head><body><h1>{}</h1><p class=\"n\">{}</p>",
        utsim_idiomas::atual().codigo(),
        esc(&title),
        esc(&(t.page_title)(&title)),
        esc(&(t.page_updated)(updated)),
    );
    let groups = kanban::open_by_person(state, project);
    if groups.is_empty() {
        h.push_str(&format!("<p>{}</p>", esc(t.page_none)));
    }
    for (who, actions) in groups {
        let name = match who {
            Some(p) => domain::person_name(state, p),
            None => t.page_no_owner.to_string(),
        };
        h.push_str(&format!(
            "<h2>{} ({})</h2><table><tr><th>{}</th><th>{}</th><th>{}</th><th>{}</th></tr>",
            esc(&name),
            actions.len(),
            esc(tc.col_action),
            esc(tc.col_column),
            esc(tc.col_due),
            esc(t.page_where),
        ));
        for a in actions {
            let due = match a.due {
                Some(d) if d < today => {
                    format!(
                        "<span class=\"late\">{}</span>",
                        esc(&(tc.late)(&d.local()))
                    )
                }
                Some(d) => esc(&d.local()),
                None => String::new(),
            };
            h.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{due}</td><td>{}</td></tr>",
                esc(&a.title),
                esc(text::column_name(a.column)),
                esc(&a.place),
            ));
        }
        h.push_str("</table>");
    }
    h.push_str("</body></html>");
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_token_and_the_host_decide() {
        assert!(allowed(Some("abc"), "192.168.0.9:8080", "t=abc", 8080));
        assert!(allowed(Some("abc"), "x", "a=1&t=abc", 8080));
        assert!(!allowed(Some("abc"), "localhost:8080", "", 8080));
        assert!(!allowed(Some("abc"), "x", "t=abd", 8080));
        assert!(!allowed(Some("abc"), "x", "t=ab", 8080));
        assert!(allowed(None, "localhost:8080", "", 8080));
        assert!(allowed(None, "127.0.0.1:8080", "", 8080));
        assert!(allowed(None, "[::1]:8080", "", 8080));
        assert!(!allowed(None, "evil.example:8080", "", 8080));
        assert!(!allowed(None, "localhost.evil.example:8080", "", 8080));
        assert!(!allowed(None, "localhost:9", "", 8080));
    }

    #[test]
    fn text_is_escaped() {
        assert_eq!(
            esc("<a href='x'>&\"</a>"),
            "&lt;a href=&#39;x&#39;&gt;&amp;&quot;&lt;/a&gt;"
        );
    }

    #[test]
    fn tokens_are_long_and_differ() {
        let a = new_token().unwrap();
        let b = new_token().unwrap();
        assert_eq!(a.len(), 32);
        assert_ne!(a, b);
    }
}

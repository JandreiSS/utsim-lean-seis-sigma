//! The gemba page (P7): served over HTTP on `127.0.0.1`, read only, with the host check when
//! closed to the network and the token when open; and the page built from a project.

use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpStream};

use utsim_lss_app::rede::{self, Gemba};
use utsim_lss_app::session::Session;
use utsim_lss_app::text::{Language, set_language};
use utsim_lss_core::domain::{Approach, Column, fields};
use utsim_lss_core::{Date, Edit, EntityRef, Id, Kind};
use utsim_lss_store::SystemClock;
use utsim_lss_store::user::Identity;

/// Status line and body of one request.
fn get(port: u16, method: &str, target: &str, host: &str) -> (String, String) {
    let mut s = TcpStream::connect((Ipv4Addr::LOCALHOST, port)).unwrap();
    write!(
        s,
        "{method} {target} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n"
    )
    .unwrap();
    let mut out = String::new();
    s.read_to_string(&mut out).unwrap();
    let (head, body) = out.split_once("\r\n\r\n").unwrap();
    (head.lines().next().unwrap().to_string(), body.to_string())
}

#[test]
fn closed_to_the_network_only_this_computer_reads() {
    set_language(Language::PtBr);
    let g = Gemba::start(false, 0).unwrap();
    assert!(!g.lan());
    let port = g.port();
    assert_eq!(g.url(), format!("http://localhost:{port}/"));
    g.set_page("<p>ações</p>".into());
    let host = format!("localhost:{port}");
    let (st, body) = get(port, "GET", "/", &host);
    assert!(st.contains("200"), "{st}");
    assert_eq!(body, "<p>ações</p>");
    // Another name for this address (DNS rebinding) is refused.
    let (st, body) = get(port, "GET", "/", &format!("site.example:{port}"));
    assert!(st.contains("403"), "{st}");
    assert!(body.contains("token"), "{body}");
    // Read only.
    assert!(get(port, "POST", "/", &host).0.contains("405"));
    assert!(get(port, "GET", "/outra", &host).0.contains("404"));
    let (st, body) = get(port, "HEAD", "/", &host);
    assert!(st.contains("200") && body.is_empty());
}

#[test]
fn with_a_token_only_the_full_address_reads() {
    set_language(Language::PtBr);
    let g = Gemba::start_on(Ipv4Addr::LOCALHOST, true, 0).unwrap();
    assert!(g.lan());
    let port = g.port();
    let url = g.url();
    let token = url.split_once("?t=").unwrap().1.to_string();
    assert_eq!(token.len(), 32);
    g.set_page("ok".into());
    let host = format!("192.168.0.20:{port}");
    assert!(get(port, "GET", "/", &host).0.contains("403"));
    assert!(get(port, "GET", "/?t=0000", &host).0.contains("403"));
    let (st, body) = get(port, "GET", &format!("/?t={token}"), &host);
    assert!(st.contains("200"), "{st}");
    assert_eq!(body, "ok");
    // Turned off, nothing answers.
    drop(g);
    assert!(TcpStream::connect((Ipv4Addr::LOCALHOST, port)).is_err());
}

#[test]
fn the_page_lists_open_actions_by_person() {
    set_language(Language::PtBr);
    let dir = tempfile::tempdir().unwrap();
    let me = Identity {
        person: Id::v7(1_700_000_000_000, [1; 10]),
        name: "Ana".into(),
        email: "ana@x.com".into(),
    };
    let mut s = Session::create(
        &dir.path().join("P"),
        "Refugo <da> prensa",
        Approach::Dmaic,
        &me,
        None,
        Box::new(SystemClock),
    )
    .unwrap();
    let a = s
        .add_card("Trocar a matriz & o pino", Column::Todo)
        .unwrap()
        .unwrap();
    let b = s.add_card("Feita", Column::Todo).unwrap().unwrap();
    let r = EntityRef::new(Kind::Action, a);
    s.apply(vec![
        Edit::new(r, fields::action::WHO, me.person.to_string()),
        Edit::new(r, fields::action::DUE, "2026-11-02"),
        Edit::new(r, fields::action::WHERE, "Prensa 3"),
        Edit::new(
            EntityRef::new(Kind::Action, b),
            fields::action::COLUMN,
            "feito",
        ),
    ])
    .unwrap();
    let today = Date::new(2026, 11, 11).unwrap();
    let html = rede::page(s.state(), s.project(), today, "10:30");
    assert!(html.contains("lang=\"pt-BR\""), "{html}");
    assert!(
        html.contains("Refugo &lt;da&gt; prensa: ações abertas"),
        "{html}"
    );
    assert!(html.contains("Atualizada em 10:30"), "{html}");
    assert!(html.contains("Trocar a matriz &amp; o pino"), "{html}");
    assert!(html.contains("Prensa 3"), "{html}");
    assert!(
        html.contains("class=\"late\">02/11/2026 (atrasada)"),
        "{html}"
    );
    assert!(!html.contains("Feita"), "done cards are left out: {html}");
    // In English the same page speaks English.
    set_language(Language::En);
    let html = rede::page(s.state(), s.project(), today, "10:30");
    assert!(
        html.contains("open actions") && html.contains("lang=\"en\""),
        "{html}"
    );
    s.close();
}

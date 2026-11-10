//! Texts of the team's signatures (P7): the signing key, the check of the signed journal and
//! the sealed approvals.

use crate::text::{Language, language};

type S = &'static str;
type F1 = fn(&str) -> String;
type F2 = fn(&str, &str) -> String;

pub struct L13Texts {
    // Key.
    pub key_title: S,
    pub key_intro: S,
    pub key_later: S,
    pub key_file: F1,
    pub key_error: F1,
    pub no_key: S,
    // Check.
    pub integrity_title: S,
    pub integrity_intro: S,
    pub recheck: S,
    pub integrity_ok: fn(usize, usize) -> String,
    pub integrity_summary: fn(usize, usize, usize, usize) -> String,
    pub nobody_signs: S,
    pub col_person: S,
    pub col_key: S,
    pub col_signed: S,
    pub col_before: S,
    pub col_links: S,
    pub details: S,
    pub chain_gap: F2,
    pub chain_start: F1,
    pub chain_order: F2,
    pub chain_fork: F2,
    pub chain_hash: F1,
    pub chain_signature: F1,
    pub chain_author: F1,
    pub chain_body: F1,
    pub altered: F2,
    pub redacted: F2,
    pub other_signer: F2,
    pub missing: F2,
    pub unsigned: F2,
    pub foreign_key: F2,
    pub key_not_proven: F2,
    pub chain_without_key: F1,
    pub new_key: fn(&str, &str, &str) -> String,
    pub severe_status: fn(usize) -> String,
    // Approvals.
    pub sign: S,
    pub sign_help: S,
    pub signed_by: F2,
    pub outdated: F1,
    pub invalid: S,
    pub not_signed: S,
    pub signed_ok: S,
    // Certificates and the role list (P9).
    pub pdf_signed: fn(&str, &str, &str, &str) -> String,
    pub roles_title: S,
    pub roles_intro: S,
    pub roles_never: S,
    pub roles_current: F2,
    pub roles_changed: fn(&str, &str, usize, usize) -> String,
    pub roles_out: F2,
    pub roles_removed: F1,
    pub roles_not_entitled: F1,
    pub roles_sign: S,
    // My actions.
    pub my_actions: fn(usize) -> String,
    pub my_actions_none: S,
    pub col_project: S,
    pub col_action: S,
    pub col_due: S,
    pub col_column: S,
    pub late: F1,
    pub open: S,
}

pub const PT_BR: L13Texts = L13Texts {
    key_title: "Chave de assinatura",
    key_intro: "As suas mudanças no diário e as suas aprovações são assinadas com esta chave, \
        guardada só neste computador. Para usar a mesma chave em outro computador, copie o \
        arquivo abaixo para a mesma pasta de lá.",
    key_later: "A chave é criada quando você continuar.",
    key_file: |p| format!("Arquivo: {p}"),
    key_error: |e| format!("Não foi possível criar a chave de assinatura: {e}"),
    no_key: "Este computador não tem chave de assinatura.",
    integrity_title: "Integridade do diário",
    integrity_intro: "Cada mudança gravada por quem tem chave entra numa cadeia assinada. A \
        conferência mostra o que foi editado depois de assinado, o que falta e o que entrou \
        sem assinatura. Ela revela alterações, mas não as impede, e não tem a validade \
        jurídica de uma assinatura digital.",
    recheck: "Conferir de novo",
    integrity_ok: |signed, total| {
        format!("{signed} de {total} mudanças assinadas, nenhum problema encontrado.")
    },
    integrity_summary: |signed, total, severe, other| {
        let mut s = format!("{signed} de {total} mudanças assinadas.");
        if severe > 0 {
            s.push_str(&format!(
                " {severe} {} de alteração.",
                if severe == 1 { "sinal" } else { "sinais" }
            ));
        }
        if other > 0 {
            s.push_str(&format!(
                " {other} {}.",
                if other == 1 { "aviso" } else { "avisos" }
            ));
        }
        s
    },
    nobody_signs: "Ninguém assina neste projeto ainda. As mudanças passam a ser assinadas \
        quando cada pessoa abre o projeto nesta versão do app.",
    col_person: "Pessoa",
    col_key: "Chave",
    col_signed: "Assinadas",
    col_before: "Antes da chave",
    col_links: "Elos",
    details: "O que a conferência encontrou",
    chain_gap: |n, after| format!("Falta parte da cadeia de {n} depois do elo {after}."),
    chain_start: |n| format!("Falta o início da cadeia de {n}."),
    chain_order: |n, seq| format!("O elo {seq} de {n} não aponta para o elo anterior."),
    chain_fork: |n, seq| {
        format!(
            "Há dois elos {seq} diferentes de {n} (uma cópia antiga da pasta restaurada e \
             usada?)."
        )
    },
    chain_hash: |n| format!("Um elo da cadeia de {n} foi editado e ficou de fora."),
    chain_signature: |n| {
        format!("Um elo da cadeia de {n} tem assinatura que não confere e ficou de fora.")
    },
    chain_author: |n| format!("Um elo na cadeia de {n} é de outra pessoa e ficou de fora."),
    chain_body: |n| format!("Um elo da cadeia de {n} não pôde ser lido."),
    altered: |n, when| format!("Uma mudança de {n} ({when}) foi editada depois de assinada."),
    redacted: |n, when| {
        format!("Uma mudança de {n} ({when}) teve o conteúdo apagado por anonimização.")
    },
    other_signer: |n, signer| {
        format!("Uma mudança em nome de {n} foi assinada na cadeia de {signer}.")
    },
    missing: |n, when| {
        format!(
            "Uma mudança assinada por {n} ({when}) não está na pasta. Pode ainda estar \
             sincronizando."
        )
    },
    unsigned: |n, when| {
        format!(
            "Uma mudança em nome de {n} ({when}) não tem assinatura: foi gravada por uma \
             versão antiga, por outro app ou por outra pessoa."
        )
    },
    foreign_key: |n, by| format!("{by} registrou uma chave em nome de {n}; ela foi ignorada."),
    key_not_proven: |n, key| {
        format!("A chave {key} de {n} não assinou o próprio registro; ela foi ignorada.")
    },
    chain_without_key: |f| format!("A pasta de cadeia {f} não tem chave registrada."),
    new_key: |n, key, when| format!("{n} passou a usar também a chave {key} em {when}."),
    severe_status: |n| {
        format!(
            "A conferência do diário encontrou {n} {} de alteração: veja Equipe.",
            if n == 1 { "sinal" } else { "sinais" }
        )
    },
    sign: "Assinar",
    sign_help: "Assina com a sua chave. A assinatura fica na pasta do projeto, e qualquer \
        pessoa da equipe pode conferi-la.",
    signed_by: |n, when| format!("✔ assinada por {n} em {when}"),
    outdated: |n| format!("⚠ a assinatura de {n} é de outra versão do conteúdo"),
    invalid: "⚠ assinatura que não confere",
    not_signed: "sem assinatura",
    signed_ok: "Assinatura gravada.",
    pdf_signed: |issuer, when, key, seal| {
        format!(
            "Assinado eletronicamente por {issuer} em {when}, com a chave {key} (selo {seal}).              A assinatura pode ser conferida no UTSim Seis Sigma, na tela Certificação do              programa. Ela não tem a validade jurídica de uma assinatura digital certificada."
        )
    },
    roles_title: "Assinatura da lista de papéis",
    roles_intro: "O master black belt assina a lista de papéis com a sua chave. Um papel         incluído depois da última assinatura, por quem quer que seja, aparece abaixo até a         lista ser assinada de novo. A primeira assinatura vale quando é do criador do         programa; as seguintes, quando são de um master black belt da lista assinada antes.",
    roles_never: "A lista de papéis ainda não foi assinada.",
    roles_current: |who, when| {
        format!("✔ Lista assinada por {who} em {when}, sem mudanças desde então.")
    },
    roles_changed: |who, when, added, removed| {
        format!(
            "⚠ A última assinatura é de {who} em {when}. Desde então: {added} {} e {removed} {}.",
            if added == 1 {
                "papel incluído"
            } else {
                "papéis incluídos"
            },
            if removed == 1 {
                "retirado"
            } else {
                "retirados"
            }
        )
    },
    roles_out: |who, profile| format!("{who}: {profile}, fora da lista assinada"),
    roles_removed: |item| format!("retirado depois da assinatura: {item}"),
    roles_not_entitled: |who| {
        format!(
            "{who} assinou a lista sem ser o criador nem master black belt da lista anterior; a assinatura não vale."
        )
    },
    roles_sign: "Assinar a lista",
    my_actions: |n| match n {
        0 => "Minhas ações em todos os projetos: nenhuma em aberto".into(),
        1 => "Minhas ações em todos os projetos: 1 em aberto".into(),
        n => format!("Minhas ações em todos os projetos: {n} em aberto"),
    },
    my_actions_none: "Nenhum cartão em aberto com você como responsável nos projetos desta         pasta.",
    col_project: "Projeto",
    col_action: "Ação",
    col_due: "Prazo",
    col_column: "Coluna",
    late: |d| format!("{d} (atrasada)"),
    open: "Abrir",
};

pub const EN: L13Texts = L13Texts {
    key_title: "Signing key",
    key_intro: "Your journal changes and your approvals are signed with this key, kept only on \
        this computer. To use the same key on another computer, copy the file below to the \
        same folder there.",
    key_later: "The key is created when you continue.",
    key_file: |p| format!("File: {p}"),
    key_error: |e| format!("Could not create the signing key: {e}"),
    no_key: "This computer has no signing key.",
    integrity_title: "Journal integrity",
    integrity_intro: "Each change saved by someone who has a key enters a signed chain. The \
        check shows what was edited after being signed, what is missing and what came in \
        without a signature. It reveals changes but does not prevent them, and it does not \
        have the legal validity of a digital signature.",
    recheck: "Check again",
    integrity_ok: |signed, total| format!("{signed} of {total} changes signed, no problem found."),
    integrity_summary: |signed, total, severe, other| {
        let mut s = format!("{signed} of {total} changes signed.");
        if severe > 0 {
            s.push_str(&format!(
                " {severe} {} of tampering.",
                if severe == 1 { "sign" } else { "signs" }
            ));
        }
        if other > 0 {
            s.push_str(&format!(
                " {other} {}.",
                if other == 1 { "warning" } else { "warnings" }
            ));
        }
        s
    },
    nobody_signs: "Nobody signs in this project yet. Changes start to be signed when each \
        person opens the project in this version of the app.",
    col_person: "Person",
    col_key: "Key",
    col_signed: "Signed",
    col_before: "Before the key",
    col_links: "Links",
    details: "What the check found",
    chain_gap: |n, after| format!("Part of {n}'s chain is missing after link {after}."),
    chain_start: |n| format!("The start of {n}'s chain is missing."),
    chain_order: |n, seq| format!("Link {seq} of {n} does not point to the previous link."),
    chain_fork: |n, seq| {
        format!(
            "There are two different links {seq} for {n} (an old copy of the folder restored \
             and used?)."
        )
    },
    chain_hash: |n| format!("A link in {n}'s chain was edited and left out."),
    chain_signature: |n| {
        format!("A link in {n}'s chain has a signature that does not match and was left out.")
    },
    chain_author: |n| format!("A link in {n}'s chain belongs to someone else and was left out."),
    chain_body: |n| format!("A link in {n}'s chain could not be read."),
    altered: |n, when| format!("A change by {n} ({when}) was edited after being signed."),
    redacted: |n, when| {
        format!("A change by {n} ({when}) had its content erased by anonymisation.")
    },
    other_signer: |n, signer| {
        format!("A change in the name of {n} was signed in {signer}'s chain.")
    },
    missing: |n, when| {
        format!(
            "A change signed by {n} ({when}) is not in the folder. It may still be \
             syncing."
        )
    },
    unsigned: |n, when| {
        format!(
            "A change in the name of {n} ({when}) has no signature: it was saved by an old \
             version, by another app or by someone else."
        )
    },
    foreign_key: |n, by| format!("{by} registered a key in the name of {n}; it was ignored."),
    key_not_proven: |n, key| {
        format!("The key {key} of {n} did not sign its own registration; it was ignored.")
    },
    chain_without_key: |f| format!("The chain folder {f} has no registered key."),
    new_key: |n, key, when| format!("{n} started to also use the key {key} on {when}."),
    severe_status: |n| {
        format!(
            "The journal check found {n} {} of tampering: see Team.",
            if n == 1 { "sign" } else { "signs" }
        )
    },
    sign: "Sign",
    sign_help: "Signs with your key. The signature is kept in the project folder, and anyone \
        on the team can check it.",
    signed_by: |n, when| format!("✔ signed by {n} on {when}"),
    outdated: |n| format!("⚠ {n}'s signature is for another version of the content"),
    invalid: "⚠ signature that does not match",
    not_signed: "not signed",
    signed_ok: "Signature saved.",
    pdf_signed: |issuer, when, key, seal| {
        format!(
            "Electronically signed by {issuer} on {when}, with key {key} (seal {seal}). \
             The signature can be checked in UTSim Seis Sigma, on the program's Certification \
             screen. It does not have the legal validity of a certified digital signature."
        )
    },
    roles_title: "Signature of the role list",
    roles_intro: "The master black belt signs the role list with their key. A role added after \
        the last signature, by anyone, appears below until the list is signed again. The first \
        signature is valid when it comes from the program's creator; the following ones, when \
        they come from a master black belt on the previously signed list.",
    roles_never: "The role list has not been signed yet.",
    roles_current: |who, when| {
        format!("✔ List signed by {who} on {when}, with no changes since then.")
    },
    roles_changed: |who, when, added, removed| {
        format!(
            "⚠ The last signature is by {who} on {when}. Since then: {added} {} and {removed} removed.",
            if added == 1 {
                "role added"
            } else {
                "roles added"
            }
        )
    },
    roles_out: |who, profile| format!("{who}: {profile}, outside the signed list"),
    roles_removed: |item| format!("removed after the signature: {item}"),
    roles_not_entitled: |who| {
        format!(
            "{who} signed the list without being the creator or a master black belt on the previous list; the signature is not valid."
        )
    },
    roles_sign: "Sign the list",
    my_actions: |n| match n {
        0 => "My actions in all projects: none open".into(),
        n => format!("My actions in all projects: {n} open"),
    },
    my_actions_none: "No open card with you as owner in the projects of this folder.",
    col_project: "Project",
    col_action: "Action",
    col_due: "Due",
    col_column: "Column",
    late: |d| format!("{d} (late)"),
    open: "Open",
};

pub const ES: L13Texts = L13Texts {
    key_title: "Clave de firma",
    key_intro: "Sus cambios en el diario y sus aprobaciones se firman con esta clave, \
        guardada solo en este computador. Para usar la misma clave en otro computador, copie el \
        archivo de abajo a la misma carpeta allí.",
    key_later: "La clave se crea cuando usted continúe.",
    key_file: |p| format!("Archivo: {p}"),
    key_error: |e| format!("No fue posible crear la clave de firma: {e}"),
    no_key: "Este computador no tiene clave de firma.",
    integrity_title: "Integridad del diario",
    integrity_intro: "Cada cambio guardado por quien tiene clave entra en una cadena firmada. \
        La verificación muestra lo que se editó después de firmado, lo que falta y lo que entró \
        sin firma. Revela alteraciones, pero no las impide, y no tiene la validez jurídica de \
        una firma digital.",
    recheck: "Verificar de nuevo",
    integrity_ok: |signed, total| {
        format!("{signed} de {total} cambios firmados, ningún problema encontrado.")
    },
    integrity_summary: |signed, total, severe, other| {
        let mut s = format!("{signed} de {total} cambios firmados.");
        if severe > 0 {
            s.push_str(&format!(
                " {severe} {} de alteración.",
                if severe == 1 { "señal" } else { "señales" }
            ));
        }
        if other > 0 {
            s.push_str(&format!(
                " {other} {}.",
                if other == 1 { "aviso" } else { "avisos" }
            ));
        }
        s
    },
    nobody_signs: "Nadie firma todavía en este proyecto. Los cambios pasan a firmarse cuando \
        cada persona abre el proyecto en esta versión de la aplicación.",
    col_person: "Persona",
    col_key: "Clave",
    col_signed: "Firmados",
    col_before: "Antes de la clave",
    col_links: "Eslabones",
    details: "Lo que encontró la verificación",
    chain_gap: |n, after| format!("Falta parte de la cadena de {n} después del eslabón {after}."),
    chain_start: |n| format!("Falta el inicio de la cadena de {n}."),
    chain_order: |n, seq| format!("El eslabón {seq} de {n} no apunta al eslabón anterior."),
    chain_fork: |n, seq| {
        format!(
            "Hay dos eslabones {seq} distintos de {n} (¿una copia antigua de la carpeta \
             restaurada y usada?)."
        )
    },
    chain_hash: |n| format!("Un eslabón de la cadena de {n} fue editado y quedó fuera."),
    chain_signature: |n| {
        format!("Un eslabón de la cadena de {n} tiene una firma que no coincide y quedó fuera.")
    },
    chain_author: |n| format!("Un eslabón de la cadena de {n} es de otra persona y quedó fuera."),
    chain_body: |n| format!("Un eslabón de la cadena de {n} no pudo leerse."),
    altered: |n, when| format!("Un cambio de {n} ({when}) fue editado después de firmado."),
    redacted: |n, when| {
        format!("Un cambio de {n} ({when}) tuvo el contenido borrado por anonimización.")
    },
    other_signer: |n, signer| {
        format!("Un cambio a nombre de {n} fue firmado en la cadena de {signer}.")
    },
    missing: |n, when| {
        format!(
            "Un cambio firmado por {n} ({when}) no está en la carpeta. Puede que aún se esté \
             sincronizando."
        )
    },
    unsigned: |n, when| {
        format!(
            "Un cambio a nombre de {n} ({when}) no tiene firma: fue guardado por una \
             versión antigua, por otra aplicación o por otra persona."
        )
    },
    foreign_key: |n, by| format!("{by} registró una clave a nombre de {n}; fue ignorada."),
    key_not_proven: |n, key| {
        format!("La clave {key} de {n} no firmó su propio registro; fue ignorada.")
    },
    chain_without_key: |f| format!("La carpeta de cadena {f} no tiene clave registrada."),
    new_key: |n, key, when| format!("{n} pasó a usar también la clave {key} el {when}."),
    severe_status: |n| {
        format!(
            "La verificación del diario encontró {n} {} de alteración: vea Equipo.",
            if n == 1 { "señal" } else { "señales" }
        )
    },
    sign: "Firmar",
    sign_help: "Firme con su clave. La firma queda en la carpeta del proyecto, y cualquier \
        persona del equipo puede verificarla.",
    signed_by: |n, when| format!("✔ firmada por {n} el {when}"),
    outdated: |n| format!("⚠ la firma de {n} es de otra versión del contenido"),
    invalid: "⚠ firma que no coincide",
    not_signed: "sin firma",
    signed_ok: "Firma guardada.",
    pdf_signed: |issuer, when, key, seal| {
        format!(
            "Firmado electrónicamente por {issuer} el {when}, con la clave {key} (sello {seal}). \
             La firma puede verificarse en UTSim Seis Sigma, en la pantalla Certificación del \
             programa. No tiene la validez jurídica de una firma digital certificada."
        )
    },
    roles_title: "Firma de la lista de roles",
    roles_intro: "El master black belt firma la lista de roles con su clave. Un rol incluido \
        después de la última firma, por quien sea, aparece abajo hasta que la lista se firme de \
        nuevo. La primera firma vale cuando es del creador del programa; las siguientes, cuando \
        son de un master black belt de la lista firmada antes.",
    roles_never: "La lista de roles aún no fue firmada.",
    roles_current: |who, when| {
        format!("✔ Lista firmada por {who} el {when}, sin cambios desde entonces.")
    },
    roles_changed: |who, when, added, removed| {
        format!(
            "⚠ La última firma es de {who} el {when}. Desde entonces: {added} {} y {removed} {}.",
            if added == 1 {
                "rol incluido"
            } else {
                "roles incluidos"
            },
            if removed == 1 {
                "retirado"
            } else {
                "retirados"
            }
        )
    },
    roles_out: |who, profile| format!("{who}: {profile}, fuera de la lista firmada"),
    roles_removed: |item| format!("retirado después de la firma: {item}"),
    roles_not_entitled: |who| {
        format!(
            "{who} firmó la lista sin ser el creador ni master black belt de la lista anterior; la firma no vale."
        )
    },
    roles_sign: "Firmar la lista",
    my_actions: |n| match n {
        0 => "Mis acciones en todos los proyectos: ninguna abierta".into(),
        1 => "Mis acciones en todos los proyectos: 1 abierta".into(),
        n => format!("Mis acciones en todos los proyectos: {n} abiertas"),
    },
    my_actions_none: "Ninguna tarjeta abierta con usted como responsable en los proyectos de \
        esta carpeta.",
    col_project: "Proyecto",
    col_action: "Acción",
    col_due: "Plazo",
    col_column: "Columna",
    late: |d| format!("{d} (atrasada)"),
    open: "Abrir",
};

pub fn l13() -> &'static L13Texts {
    match language() {
        Language::PtBr => &PT_BR,
        Language::En => &EN,
        Language::Es => &ES,
    }
}

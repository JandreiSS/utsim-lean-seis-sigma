//! Texts of the team's signatures (P7): the signing key, the check of the signed journal and
//! the sealed approvals.

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
    signed_by: |n, when| format!("✓ assinada por {n} em {when}"),
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
        format!("✓ Lista assinada por {who} em {when}, sem mudanças desde então.")
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

pub fn l13() -> &'static L13Texts {
    &PT_BR
}

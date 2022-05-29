use super::*;

pub type EmailOptionFn = fn(&mut Body, Request);

pub struct Request<'a> {
    pub val: &'a str,
    pub token: Option<&'a models::Token>,
    pub board: Option<&'a models::Board>,
    pub thread: Option<&'a models::Thread>,
}

pub struct Body {
    pub subject: Option<Box<str>>,
    pub email: Option<Box<str>>,
    pub name: Option<Box<str>>,
    pub comment: Box<str>,
    pub sage: bool,
    pub op: bool,
    pub good: u32,
    pub bad: u32,
}

lazy_static! {
    static ref EMAIL_OPTIONS_SETTINGS: HashMap<&'static str, EmailOptionFn> = hashmap! {
        "sage" => fn_sage as EmailOptionFn,
        "op"   => fn_op   as EmailOptionFn,
        "rage" => fn_rage as EmailOptionFn,
    };
}

pub fn fn_sage(body: &mut Body, _req: Request) {
    body.sage = true;
}

pub fn fn_rage(body: &mut Body, _req: Request) {
    body.name = Some(
        format!(
            "<span class=\"rage\">{}</span>",
            match &body.name {
                Some(v) => &v,
                None => "RAGE",
            }
        )
        .into(),
    )
}

#[test]
fn test_sage() {
    let mut body = Body::default();
    body.email = Some("sage".into());
    body.apply(None, None, None);
    assert!(body.sage);
}

pub fn fn_op(body: &mut Body, req: Request) {
    if let Some(b) = req.board {
        if !b.op_oppost_enabled {
            return;
        }
    }
    let secret: u64 = match req.val.parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    match req.thread {
        Some(v) => {
            if v.secret == secret {
                body.op = true
            }
        }
        None => (),
    }
}

#[test]
fn test_op() {
    let mut body = Body::default();
    body.email = Some("op=100".into());

    body.apply(
        None,
        None,
        Some(&models::Thread {
            id: 0,
            board_id: 0,
            ip: 0,
            count: 0,
            post_id: 0,
            post_num: 0,
            sticky: 0,
            lasthit: chrono::Utc::now().naive_utc(),
            posts: None,
            owner: None,
            secret: 100,
            closed: 0,
        }),
    );

    assert!(body.op);
}

#[test]
fn test_sage_op() {
    let mut body = Body::default();
    body.email = Some("op=100,sage".into());

    body.apply(
        None,
        None,
        Some(&models::Thread {
            id: 0,
            board_id: 0,
            ip: 0,
            count: 0,
            post_id: 0,
            post_num: 0,
            sticky: 0,
            lasthit: chrono::Utc::now().naive_utc(),
            posts: None,
            owner: None,
            secret: 100,
            closed: 0,
        }),
    );

    assert!(body.op);
    assert!(body.sage);
}

impl Default for Body {
    fn default() -> Self {
        Self {
            subject: None,
            email: None,
            name: None,
            comment: "".into(),
            sage: false,
            op: false,
            good: 0,
            bad: 0,
        }
    }
}

impl Body {
    pub fn apply(
        &mut self,
        token: Option<&models::Token>,
        board: Option<&models::Board>,
        thread: Option<&models::Thread>,
    ) {
        // get email
        let email = match self.email.take() {
            Some(v) => v,
            None => return,
        };

        // check if it's correct email
        if crate::REGEX_EMAIL.is_match(&email) {
            return;
        } else {
            self.email = None
        };

        // parse args
        let split = email.split(',');

        for raw in split {
            if raw.len() == 0 {
                continue;
            }

            let mut key = String::new();
            let mut val = String::new();
            let mut parsing_val = false;
            let mut quotes = false;

            for ch in raw.chars() {
                match ch {
                    '\'' => quotes = !quotes,

                    '=' if quotes == false => {
                        if parsing_val {
                            break;
                        } else {
                            parsing_val = true
                        }
                    }

                    _ => {
                        if parsing_val {
                            &mut val
                        } else {
                            &mut key
                        }
                    }
                    .push(ch),
                }
            }

            // get fn
            match EMAIL_OPTIONS_SETTINGS.get(key.as_str()) {
                Some(v) => v(
                    self,
                    Request {
                        val: &val,
                        token: token,
                        board: board,
                        thread: thread,
                    },
                ),

                None => (),
            };
        }
    }
}

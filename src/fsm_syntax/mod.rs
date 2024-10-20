pub struct Header<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

pub struct StateSpec<'a> {
    pub name: &'a str,
    pub super_state: &'a str,
    pub entry_action: &'a str,
    pub exit_action: &'a str,
    pub abstract_state: bool,
}

pub struct SubTransitions<'a> {
    event: &'a str,
    next_state: &'a str,
    actions: Vec<&'a str>,
}

pub struct Transition<'a> {
    pub state: StateSpec<'a>,
    pub sub_transitions: Vec<SubTransitions<'a>>,
}

pub struct SyntaxError<'a> {
    pub error_type: &'a str,
    pub message: &'a str,
}

pub struct FsmSyntax<'a> {
    pub headers: Vec<Header<'a>>,
    pub logic: Vec<Transition<'a>>,
    pub errors: Vec<SyntaxError<'a>>,
    pub done: bool,
}

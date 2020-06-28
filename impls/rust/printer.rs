use crate::types::Types;

fn wrap_list(v: Vec<Types>) -> String {
    "(".to_string()
        + &v[1..v.len() - 1]
            .iter()
            .map(|x| pr_str(x.clone()))
            .collect::<Vec<String>>()
            .join(" ")
        + ")"
}

pub fn pr_str(t: Types) -> String {
    match t {
        Types::Str(s) => format!("{}", s),
        Types::Int(i) => format!("{}", i),
        Types::Bool(b) => format!("{}", b),
        Types::Sym(x) => format!("{}", x),
        Types::Nil => "nil".to_string(),
        Types::List(v) => wrap_list(v),
        Types::EOF => "EOF".to_string(),
    }
}

pub fn pr_debug(t: Types) -> String {
    format!("{:?}", t)
}

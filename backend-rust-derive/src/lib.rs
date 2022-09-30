use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::Nothing,
    parse_macro_input,
    spanned::Spanned,
    visit::{self, Visit},
    Error, Item, ItemFn, Pat, PatIdent, PatType, Type, TypePath,
};

/*
struct Struct1 {
    val1: Type1,
    val2: Type2
}
-->
static Struct1_typescript: &str = r#"
type Struct1 {
    val1: Type1,
    val2: Type2
}
"#;

#[post("/login")]
async fn login(
    session: Session,
    tucan: web::Data<Tucan>,
    login: web::Json<Login>,
) -> Result<Json<LoginResult>>, MyError> {
    let tucan_user = tucan.login(&login.username, &login.password).await?;
    session.insert("session", tucan_user.session).unwrap();
    Ok(web::Json(LoginResult { success: true }))
}
-->
static login_typescript: &str = r#"
async function login(login: Login): LoginResult {
    fetch("/login")...
}
"#;
*/

/*
if syn is epic it may be possible to somehow extract the data based on a ref to the other types
*/

/*
typescript_app!(app, index, login, logout, get_modules)

->

app.service(index).service(login).service(logout),....

write_to_file(login_typescript, ...)
*/

fn type_to_string(the_type: &Box<Type>) -> String {
    match &**the_type {
        Type::Path(TypePath { path, .. }) => format!(
            "{:?}",
            path.segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("_")
        ),
        _ => panic!(),
    }
}

fn pat_to_string(pat: &Box<Pat>) -> String {
    match &**pat {
        Pat::Ident(PatIdent { ident, .. }) => ident.to_string(),
        _ => panic!(),
    }
}

struct FnVisitor(Option<TokenStream>);

impl<'ast> Visit<'ast> for FnVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        println!("Function with name={}", node.sig.ident);
        let return_type = match node.sig.output {
            syn::ReturnType::Default => "void".to_string(),
            syn::ReturnType::Type(_, ref path) => type_to_string(path),
        };
        println!("Function with return type={}", return_type);

        let args = node
            .sig
            .inputs
            .iter()
            .map(|input| match input {
                syn::FnArg::Receiver(_) => todo!(),
                syn::FnArg::Typed(PatType { pat, ty, .. }) => {
                    (pat_to_string(pat), type_to_string(ty))
                }
            })
            .collect::<Vec<_>>();

        println!("Function with args={:?}", args);

        let test = node.sig.ident.to_string();
        self.0 = Some(quote! {
           static Struct1_typescript: &str = #test;
        });
    }
}

fn typescript_impl(input: Item) -> TokenStream {
    let typescript = match &input {
        Item::Fn(function) => {
            let mut visitor = FnVisitor(None);
            visitor.visit_item_fn(&function);
            visitor.0.unwrap()
        },
        Item::Struct(structure) => {
            quote! {
                
            }
        }
        wrong_item => {
            return Error::new(wrong_item.span(), "expected function or struct").to_compile_error()
        }
    };

    quote! {
        #typescript
        #input
    }
}

#[proc_macro_attribute]
pub fn typescript(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(attr as Nothing);
    let input = parse_macro_input!(item as Item);

    proc_macro::TokenStream::from(typescript_impl(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

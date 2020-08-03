use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// Iterator comprehension
///
/// The syntax is similar to [Haskell's list comprehension](https://wiki.haskell.org/List_comprehension).
///
/// Basic syntax is as: `[<expr>; <quals>, ...]`
///
/// # Examples
///
/// `<pat> <- <expr>` binds items of `expr` to `pat`.
/// `expr` must have `.into_iter()` method.
///
/// ```
/// # use comprehension::iter;
/// iter![x * x; x <- 0..10];
/// // => [0, 1, 4, 9, 16, 25, 36, 49, 64, 81]
/// ```
///
/// You can also use patterns.
///
/// ```
/// # use comprehension::iter;
/// iter![x * y; (x, y) <- vec![(1, 1), (2, 3), (4, 5)]];
/// // => [1, 6, 20]
/// ```
///
/// `<expr>` filters item.
/// `expr` must have type `bool`.
///
/// ```
/// # use comprehension::iter;
/// fn gcd(a: i32, b: i32) -> i32 {
///     if b == 0 { a } else { gcd(b, a % b) }
/// }
///
/// iter![(i, j); i <- 1.., j <- 1..i, gcd(i, j) == 1].take(10);
/// // => [(1, 1), (2, 1), (2, 2), (2, 3), (2, 4), (3, 1), (3, 2), (3, 3), (3, 4), (3, 5)]
/// ```
///
/// `let <pat> = <expr>` introduces a binding.
///
/// ```
/// # use comprehension::iter;
/// iter![(i, j); i <- 1.., let k = i * i, j <- 1..=k].take(10);
/// // => [(1, 1), (2, 1), (2, 2), (2, 3), (2, 4), (3, 1), (3, 2), (3, 3), (3, 4), (3, 5)]
/// ```
///
/// If there is no binding to iterator, just one element will be returned (same as Haskell's behaviour).
///
/// ```
/// # use comprehension::iter;
/// iter![1; ];      // => [1]
/// iter![1; false]; // => []
/// iter![1; true];  // => [1]
/// ```
///
#[proc_macro]
pub fn iter(item: TokenStream) -> TokenStream {
    let comp = parse_macro_input!(item as Comprehension);

    let body = comp.body;
    let mut ret = quote! {
        std::iter::once(#body)
    };

    for q in comp.quals.iter().rev() {
        match q {
            Qual::Generator(pat, iter) => {
                ret = quote! {
                    (#iter).into_iter().flat_map(move |#pat| #ret)
                };
            }
            Qual::LocalDecl(expr_let) => {
                ret = quote! {
                    {
                        #expr_let;
                        #ret
                    }
                };
            }
            Qual::Guard(pred) => {
                ret = quote! {
                    std::iter::once(())
                        .take(if #pred {1} else {0})
                        .flat_map(move |_| #ret)
                }
            }
        }
    }
    ret.into()
}

struct Comprehension {
    body: syn::Expr,
    quals: Vec<Qual>,
}

enum Qual {
    Generator(syn::Pat, syn::Expr),
    LocalDecl(syn::ExprLet),
    Guard(syn::Expr),
}

impl syn::parse::Parse for Comprehension {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::{punctuated::Punctuated, Token};

        let body = input.parse()?;
        input.parse::<syn::Token![;]>()?;
        let quals = Punctuated::<Qual, Token![,]>::parse_terminated(input)?
            .into_iter()
            .collect();
        Ok(Comprehension { body, quals })
    }
}

impl syn::parse::Parse for Qual {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        parse_generator(input)
            .or_else(|_| parse_local_decl(input))
            .or_else(|_| parse_guard(input))
    }
}

fn parse_generator(input: syn::parse::ParseStream) -> syn::Result<Qual> {
    if {
        let input = input.fork();
        input
            .parse::<syn::Pat>()
            .and_then(|_| input.parse::<syn::Token![<-]>())
            .is_ok()
    } {
        let pat = input.parse()?;
        input.parse::<syn::Token![<-]>()?;
        let expr = input.parse()?;
        Ok(Qual::Generator(pat, expr))
    } else {
        Err(syn::Error::new(input.span(), "expect pat"))
    }
}

fn parse_local_decl(input: syn::parse::ParseStream) -> syn::Result<Qual> {
    if input.peek(syn::Token![let]) {
        input.parse().map(Qual::LocalDecl)
    } else {
        Err(syn::Error::new(input.span(), "expect `let`"))
    }
}

fn parse_guard(input: syn::parse::ParseStream) -> syn::Result<Qual> {
    input.parse().map(Qual::Guard)
}

/// Vector comprehension
///
/// `vect![...]` just is same as `iter![...].collect::<Vec<_>>()`
///
#[proc_macro]
pub fn vect(item: TokenStream) -> TokenStream {
    let body: proc_macro2::TokenStream = iter(item).into();
    let ret = quote! {
        #body.collect::<Vec<_>>()
    };
    ret.into()
}

/// Sum of iterator comprehension
///
/// `sum![...]` is same as `iter![...].sum()` excepting output type will be inferred.
///
/// ```
/// # use comprehension::sum;
/// let s = sum![i; i <- 1..=10]; // this compiles
/// // let s = iter![i; i <- 1..=10].sum(); // this does not compile
/// ```
///
#[proc_macro]
pub fn sum(item: TokenStream) -> TokenStream {
    let body: proc_macro2::TokenStream = iter(item).into();
    let ret = quote! {
        {
            fn sum_helper<T, I>(it: I) -> T
            where
                T: std::iter::Sum<T>,
                I: Iterator<Item = T>,
            {
                it.sum()
            }
            sum_helper(#body)
        }
    };
    ret.into()
}

/// Product of iterator comprehension
///
/// `product![...]` is same as `iter![...].product()` excepting output type will be inferred.
///
/// ```
/// # use comprehension::product;
/// let s = product![i; i <- 1..=10]; // this compiles
/// // let s = iter![i; i <- 1..=10].product(); // this does not compile
/// ```
///
#[proc_macro]
pub fn product(item: TokenStream) -> TokenStream {
    let body: proc_macro2::TokenStream = iter(item).into();
    let ret = quote! {
        {
            fn product_helper<T, I>(it: I) -> T
            where
                T: std::iter::Product<T>,
                I: Iterator<Item = T>,
            {
                it.product()
            }
            product_helper(#body)
        }
    };
    ret.into()
}

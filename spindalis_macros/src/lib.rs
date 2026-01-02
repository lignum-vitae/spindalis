use proc_macro::TokenStream;
use std::str::FromStr;
use quote::quote;
use syn::{parse_macro_input, LitStr, LitFloat, Token, parse::Parse, parse::ParseStream, Result};

// Polynomial Parsing Macros

#[proc_macro]
pub fn parse_simple_polynomial(input: TokenStream) -> TokenStream {
    let output =
        match spindalis_core::polynomials::simple::parse_simple_polynomial(input.to_string()) {
            Ok(terms) => terms,
            Err(e) => {
                let error_msg = format!("{:?}", e);
                let error_tokens =
                    format!("compile_error!(\"{}\")", error_msg.replace("\"", "\\\""));
                return TokenStream::from_str(&error_tokens).unwrap();
            }
        };

    let mut tokens = String::from("vec![");
    for coeff in output {
        tokens.push_str(&format!("{coeff:?},"));
    }
    tokens.push(']');

    TokenStream::from_str(&tokens).unwrap()
}

#[proc_macro]
pub fn parse_polynomial_extended(input: TokenStream) -> TokenStream {
    let output =
        match spindalis_core::polynomials::extended::parse_polynomial_extended(input.to_string()) {
            Ok(terms) => terms,
            Err(e) => {
                let error_msg = format!("{:?}", e);
                let error_tokens =
                    format!("compile_error!(\"{}\")", error_msg.replace("\"", "\\\""));
                return TokenStream::from_str(&error_tokens).unwrap();
            }
        };

    let mut tokens = String::from("vec![");
    for term in output {
        tokens.push_str(&format!(
            "::spindalis_core::polynomials::Term {{ coefficient: {:?}, variables: vec![",
            term.coefficient,
        ));
        for (var, pow) in term.variables {
            tokens.push_str(&format!("(\"{var}\".to_string(), {pow:?}),"));
        }
        tokens.push_str("] },");
    }
    tokens.push(']');

    TokenStream::from_str(&tokens).unwrap()
}







struct IntegralArgs {
    poly_str: String,
    lower: f64,
    upper: f64,
}

// Parses: ("3x^2 + 2", 0.0, 1.0)
impl Parse for IntegralArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let poly_str: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let lower: LitFloat = input.parse()?;
        input.parse::<Token![,]>()?;
        let upper: LitFloat = input.parse()?;
        
        Ok(IntegralArgs {
            poly_str: poly_str.value(),
            lower: lower.base10_parse()?,
            upper: upper.base10_parse()?,
        })
    }
}

#[proc_macro]
pub fn definite_integral(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as IntegralArgs);

    //  Parse string to coefficients using your existing core logic
    let coefficients = match spindalis_core::polynomials::simple::parse_simple_polynomial(args.poly_str) {
        Ok(coeffs) => coeffs.coefficients,
        Err(e) => {
            let err = format!("Compile-time Polynomial Parse Error: {:?}", e);
            return quote!(compile_error!(#err)).into();
        }
    };

    //  Calculate the result using your analytical_integral function
    let result = spindalis_core::integrals::analytical_integral(
        &coefficients, 
        args.lower, 
        args.upper
    );

    //  Emit the final result as a float literal
    let expanded = quote! {
        #result
    };

    TokenStream::from(expanded)
}
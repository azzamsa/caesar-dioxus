use dioxus::prelude::*;

use crate::rot::{rot13, rot13_letter, Mode};

#[derive(Clone, PartialEq)]
pub struct SecretState {
    input: String,
    output: String,
}

impl SecretState {
    fn new() -> Self {
        Self {
            input: "".to_string(),
            output: "".to_string(),
        }
    }
    fn encrypt(&mut self, input: String) {
        let input_length = input.len();
        let input_bytes = input.as_bytes();
        self.input = input.clone();

        self.output = if input_length == 1 {
            let byte_result = rot13_letter(Mode::Encrypt, input_bytes[0]);
            format!("{}", String::from_utf8_lossy(&[byte_result]))
        } else {
            let bytes_result = rot13(Mode::Encrypt, input_bytes);
            format!("{}", String::from_utf8_lossy(&bytes_result))
        };
    }
    fn decrypt(&mut self, output: String) {
        let output_length = output.len();
        let output_bytes = output.as_bytes();
        self.output = output.clone();

        self.input = if output_length == 1 {
            let byte_result = rot13_letter(Mode::Decrypt, output_bytes[0]);
            format!("{}", String::from_utf8_lossy(&[byte_result]))
        } else {
            let bytes_result = rot13(Mode::Decrypt, output_bytes);
            format!("{}", String::from_utf8_lossy(&bytes_result))
        };
    }
}

pub fn dashboard(cx: Scope) -> Element {
    let secret = use_state(&cx, SecretState::new);

    cx.render(rsx!(
        section { class: "flex flex-col mt-10 ",
                  input_textarea(secret: secret.clone()),
                  div { class: "flex justify-center" }
                  output_textarea(secret: secret.clone()),
            }
    ))
}

#[derive(PartialEq, Props)]
pub struct SecretProps {
    secret: UseState<SecretState>,
}

pub fn input_textarea(cx: Scope<SecretProps>) -> Element {
    let secret = &cx.props.secret;

    cx.render(rsx!(
        div { class: "mb-6 pt-3 rounded bg-gray-200",
              id: "input",
              label { class: "input-label",
                      "Secret",
              },
              textarea { class: "input",
                         placeholder: "me@casar.tld",
                         value: "{secret.input}",
                         oninput: move |e| {
                             secret.make_mut().encrypt(e.value.clone());
                         }
              }
            }
    ))
}

pub fn output_textarea(cx: Scope<SecretProps>) -> Element {
    let secret = &cx.props.secret;

    cx.render(rsx!(
        div { class: "mb-6 pt-3 rounded bg-gray-200",
              label { class: "input-label",
                      "Encrypted",
              },
              textarea { class: "input",
                         placeholder: "zr@pnfne.gyq",
                         value: "{secret.output}",
                         oninput: move |e| {
                             secret.make_mut().decrypt(e.value.clone());
                         }

              }
            }
    ))
}

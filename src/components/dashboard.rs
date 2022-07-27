use dioxus::{events::*, prelude::*};
use nrot::{rot, rot_letter, Mode};

fn encrypt(plain: String) -> String {
    let rotation = 13;
    let plain_length = plain.len();
    let plain_bytes = plain.as_bytes();

    if plain_length == 1 {
        let byte_result = rot_letter(Mode::Encrypt, plain_bytes[0], rotation);
        format!("{}", String::from_utf8_lossy(&[byte_result]))
    } else {
        let bytes_result = rot(Mode::Encrypt, plain_bytes, rotation);
        format!("{}", String::from_utf8_lossy(&bytes_result))
    }
}

fn decrypt(secret: String) -> String {
    let rotation = 13;
    let secret_length = secret.len();
    let secret_bytes = secret.as_bytes();

    if secret_length == 1 {
        let byte_result = rot_letter(Mode::Decrypt, secret_bytes[0], rotation);
        format!("{}", String::from_utf8_lossy(&[byte_result]))
    } else {
        let bytes_result = rot(Mode::Decrypt, secret_bytes, rotation);
        format!("{}", String::from_utf8_lossy(&bytes_result))
    }
}

pub fn dashboard(cx: Scope) -> Element {
    let plain = use_state(&cx, || "".to_string());
    let secret = use_state(&cx, || "".to_string());

    let on_input_plain = move |e: FormEvent| {
        plain.set(e.value.clone());
        let secret_str = encrypt(e.value.clone());
        secret.set(secret_str);
    };
    let on_input_secret = move |e: FormEvent| {
        secret.set(e.value.clone());
        let plain_str = decrypt(e.value.clone());
        plain.set(plain_str);
    };

    cx.render(rsx!(
        section { class: "flex flex-col mt-10 ",
                  div { class: "mb-6 pt-3 rounded bg-gray-200",
                        label { class: "input-label",
                                "Plain",
                        },
                        // plain textarea
                        textarea { class: "input",
                                   placeholder: "me@casar.tld",
                                   value: "{plain}",
                                   oninput: on_input_plain
                        }
                  }
                  div { class: "flex justify-center" }
                  div { class: "mb-6 pt-3 rounded bg-gray-200",
                        label { class: "input-label",
                                "Plain",
                        },
                        // secret textarea
                        textarea { class: "input",
                                   placeholder: "me@casar.tld",
                                   value: "{secret}",
                                   oninput: on_input_secret
                        }
                  }
        }
    ))
}

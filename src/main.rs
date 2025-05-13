use dioxus::{prelude::*, Ok};

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Main {}
    }
}

#[component]
pub fn Main() -> Element {

    let mut toggle_form = use_signal(|| false);
    let mut button_text = use_signal(|| String::new());

    let mut first_name = use_signal(|| String::new());
    let mut last_name = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    if toggle_form() == false {
        button_text.set(String::from("Create Account"));
    } else {
        button_text.set(String::from("Signin"));
    }   

    rsx! {
        div {
            class: "login-container",

            if toggle_form() == false {
                div { 
                    h1 { "Create an account" },
                    div {
                        class  : "row sub-text-container",
                        p { "Already have an account? " },
                        a { 
                            onclick : move |_| {
                                toggle_form.set(true)
                            },
                            "Login"
                        }
                    }
                }
            }else{
                div { 
                    h1 { "Signin" },
                    div {
                        class  : "row sub-text-container",
                        p { "Don't have an account? " },
                        a { 
                            onclick : move |_| {
                                toggle_form.set(false)
                            },
                            "Signup"
                        }
                    }
                }
            }

            if toggle_form() == false {
                div {
                    class: "row col2",
                    div {
                        input { 
                            type : "text",
                            placeholder: "First Name",
                            oninput: move |event| {
                                first_name.set(event.value());
                            },
                        }
                    }
                    div {
                        input { 
                            type : "text",
                            placeholder: "Last Name",
                            oninput: move |event| {
                                last_name.set(event.value());
                            },
                        }
                    }
                },
            }

            div {
                class: "row",
                input { 
                    type : "text",
                    placeholder: "Email",
                    oninput: move |event| {
                        email.set(event.value());
                    },
                }
            },
            div {
                class: "row",
                input { 
                    type : "password",
                    placeholder: "Password",
                    oninput: move |event| {
                        password.set(event.value());
                    },
                } 
            }
            div {
                button {
                    class : "btn-primary",
                    onclick : move |_| async move {
                        if toggle_form() == false {
                            // Register user
                            register(first_name(), last_name(), email(), password()).await.unwrap();
                        } else {
                            // Signin user
                        }
                    },
                    {button_text()}
                }
            },
            fieldset { 
                legend { "Or signin with" }
                div {
                    class: "row",
                    button {
                        class : "btn-signin-github",
                        onclick : move |_| {
                            // Signin with google
                        },
                        img {
                            src: asset!("/assets/google-icon.svg")
                        },
                        "Sign in with Google"
                    }
                }
            }
        }
    }
}

/// Echo the user input on the server.
#[server()]
async fn register(first_name: String, last_name : String, email : String, password : String) -> Result<(), ServerFnError> {

    // use rusqlite::Connection;

    // let con: Connection = Connection::open("./users.db").unwrap();

    // con.execute(
    //     "CREATE TABLE IF NOT EXISTS users (
    //         id INTEGER PRIMARY KEY,
    //         first_name TEXT NOT NULL,
    //         last_name TEXT NOT NULL,
    //         email TEXT NOT NULL,
    //         password TEXT NOT NULL,
    //         provider TEXT NOT NULL
    //     )",
    //     [],
    // ).unwrap();

    // let mut row = con.query_row("SELECT email FROM users WHERE email = ?1 AND provider = 'classic'", &[&email], |row| {
    //     row.get::<usize, String>(0)
    // }).unwrap();

    //Err(QueryReturnedNoRows)
    // let mesage = match row {
    //     Err(ref e) => {
    //         println!("No : {:?}", row);
    //         "User created".to_string()
    //     },
    //     _ => { "User exists".to_string() }
    // };

    // println!("User : {:?}", row);

    //Ok(mesage)
    Ok(())

    // use argon2::Argon2;
    // use argon2::password_hash::{
    //     PasswordHasher, 
    //     PasswordVerifier,
    //     SaltString,
    //     rand_core::OsRng
    // };

    // let argon2 = Argon2::default(); 

    // let salt = SaltString::generate(&mut OsRng);  

    // let password_hash = argon2.hash_password(password.as_bytes(), &salt)
    //     .unwrap()
    //     .to_string();

    // println!("{}", password_hash);

    // let mut stmt = con.prepare("INSERT INTO users 
    //     (first_name, last_name, email, password, provider) 
    //     VALUES (?, ?, ?, ?, ?)").unwrap();
    
    // stmt.execute(&[
    //     &first_name, 
    //     &last_name, 
    //     &email, 
    //     &password_hash,
    //     "classic"
    // ]).unwrap();

    //Ok(())
}

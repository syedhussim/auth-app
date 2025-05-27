use dioxus::{prelude::*};

#[cfg(feature = "server")]
use axum::extract::Query;

const MAIN_CSS: Asset = asset!("/assets/main.css");

const GOOGLE_REDIRECT_URL : &'static str = "http://127.0.0.1:8080/google_auth";
const GOOGLE_CLIENT_ID : &'static str = "";

#[cfg(feature = "server")]
const GOOGLE_CLIENT_SECRET : &'static str = "";

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {

    use rusqlite::Connection;
    let con: Connection = Connection::open("./users.db").unwrap();

    con.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            email TEXT NOT NULL,
            password TEXT,
            provider TEXT NOT NULL
        )",
        [],
    ).unwrap();


    let address = dioxus::cli_config::fullstack_address_or_localhost();

    let router = axum::Router::new()
        .route("/google_auth", axum::routing::get(google_auth))
        .serve_dioxus_application(ServeConfigBuilder::default(), App);

    let router = router.into_make_service();
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

#[cfg(not(feature = "server"))]
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
    let mut response_msg = use_signal(|| String::new());

    let mut first_name = use_signal(|| String::new());
    let mut last_name = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    //...
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

            div { 
                class : "msg",
                "{response_msg}" 
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
                            let response_text = register(first_name(), last_name(), email(), password()).await.unwrap();

                            response_msg.set(response_text);
                        } else {
                            // Signin user
                            let response_text = login(email(), password()).await.unwrap();

                            response_msg.set(response_text);
                        }
                    },
                    {button_text()}
                }
            },
            fieldset { 
                legend { "Or signin with" }
                div {
                    class: "row",
                    a {
                        class : "btn-signin-google",
                        href : format!("https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=email profile", GOOGLE_CLIENT_ID, GOOGLE_REDIRECT_URL),
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

#[server()]
async fn register(
    first_name : String, 
    last_name : String, 
    email : String, 
    password : String) -> Result<String, ServerFnError> {

    use rusqlite::Connection;
    let con: Connection = Connection::open("./users.db").unwrap();

    let mut row = con.query_row("SELECT email FROM users 
        WHERE email = ?1 AND provider = 'classic'", &[&email], |row| {
        row.get::<usize, String>(0)
    });

    let message = match row {
        Err(ref e) => {
            // Create User
            use argon2::Argon2;
            use argon2::password_hash::{
                PasswordHasher, 
                SaltString,
                rand_core::OsRng
            };

            let argon2 = Argon2::default(); 

            let salt = SaltString::generate(&mut OsRng);  

            let password_hash = argon2.hash_password(password.as_bytes(), &salt)
                .unwrap()
                .to_string();

            let mut stmt = con.prepare("INSERT INTO users 
                (first_name, last_name, email, password, provider) 
                VALUES (?, ?, ?, ?, ?)").unwrap();
            
            stmt.execute(&[
                &first_name, 
                &last_name, 
                &email, 
                &password_hash,
                "classic"
            ]).unwrap();

            { "User created." }
        },
        Ok(_) => { 
            "A user with this email already exists." 
        }
    };

    Ok(message.to_string())
}

#[server()]
async fn login(
    email : String, 
    password : String) -> Result<String, ServerFnError> {

    use rusqlite::Connection;
    let con: Connection = Connection::open("./users.db").unwrap();

    let mut row = con.query_row("SELECT id, first_name, last_name, password 
        FROM users WHERE email = ?1 AND provider = 'classic'", &[&email], |row| {
        Ok(
            (
                row.get::<usize, usize>(0).unwrap(),
                row.get::<usize, String>(1).unwrap(),
                row.get::<usize, String>(2).unwrap(),
                row.get::<usize, String>(3).unwrap()
            )
        )
    });

    let message = match row {
        Ok(user) => {
            
            use argon2::Argon2;
            use argon2::password_hash::{
                PasswordHash, 
                PasswordVerifier
            };

            let argon2 = Argon2::default();

            let hash = PasswordHash::new(&user.3).unwrap();

            let result = argon2.verify_password(password.as_bytes(), &hash);

            if result.is_ok(){
                "Login successful"
            }else{
                "Invalid login details"
            }
        },
        Err(ref e) => {
            "Invalid login details"
        }, 
    };

    Ok(message.to_string())
}

#[cfg(feature = "server")]
pub async fn google_auth(Query(params) : Query<std::collections::HashMap<String, String>>) -> &'static str {

    if let Some(code) = params.get("code"){

        use std::collections::HashMap;

        let mut form = HashMap::new();
        form.insert("code", code.as_str());
        form.insert("client_id", GOOGLE_CLIENT_ID);
        form.insert("client_secret", GOOGLE_CLIENT_SECRET);
        form.insert("redirect_uri", GOOGLE_REDIRECT_URL);
        form.insert("grant_type", "authorization_code");

        let client = reqwest::Client::new();

        let response = client.post("https://oauth2.googleapis.com/token")
            .form(&form)
            .send()
            .await
            .unwrap();

        let data : Token = response.json().await.unwrap();

        let info_response = client
            .get("https://www.googleapis.com/oauth2/v3/userinfo") 
            .bearer_auth(data.access_token)
            .send()
            .await
            .unwrap();

        let user : User = info_response.json().await.unwrap();

        use rusqlite::Connection;
        let con: Connection = Connection::open("./users.db").unwrap();

        let mut row = con.query_row("SELECT id, first_name, last_name 
            FROM users WHERE email = ?1 AND provider = 'google'", &[&user.email], |row| {
            Ok(
                (
                    row.get::<usize, usize>(0).unwrap(),
                    row.get::<usize, String>(1).unwrap(),
                    row.get::<usize, String>(2).unwrap()
                )
            )
        });

        match row {
            Ok(user) => {
                println!("Login successful");
            },
            Err(ref e) => {

                let mut stmt = con.prepare("INSERT INTO users 
                    (first_name, last_name, email, provider) 
                    VALUES (?, ?, ?, ?)").unwrap();
                
                stmt.execute(&[
                    &user.given_name, 
                    &user.family_name, 
                    &user.email, 
                    "google"
                ]).unwrap();

                // Continue with login
            }, 
        };
    }

    "Process complete"
}

#[derive(Debug, serde::Deserialize)]
pub struct Token {
    access_token : String
}

#[derive(Debug, serde::Deserialize)]
pub struct User{
    given_name : String,
    family_name : String, 
    email : String
}
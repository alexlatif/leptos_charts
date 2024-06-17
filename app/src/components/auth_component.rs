use leptos::*;
use leptos::{ReadSignal, WriteSignal};
use std::default::Default;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Clone, Default)]
pub struct User {
    pub username: String,
    pub logged_in: bool,
}

#[derive(Clone)]
pub struct AuthState {
    pub user: (ReadSignal<User>, WriteSignal<User>),
}

impl Default for AuthState {
    fn default() -> Self {
        let (user, set_user) = create_signal(User::default());
        Self { user: (user, set_user) }
    }
}

impl AuthState {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn login(&self, username: String) {
        self.user.1.set(User { username, logged_in: true });
    }

    pub fn logout(&self) {
        self.user.1.set(User::default());
    }
}

#[component]
pub fn AuthComponent(auth_state: Rc<RefCell<AuthState>>) -> impl IntoView {
    let (username, set_username) = create_signal(String::new());

    let auth_state_clone = auth_state.clone();
    let on_login = move |_| {
        auth_state_clone.borrow().login(username.get().clone());
    };

    let auth_state_clone = auth_state.clone();
    let on_logout = move |_| {
        auth_state_clone.borrow().logout();
    };

    let render_auth_state = move || {
        if auth_state.borrow().user.0.get().logged_in {
            view! {
                <div>
                    <p>{format!("Welcome, {}", auth_state.borrow().user.0.get().username)}</p>
                    <button on:click=on_logout>"Logout"</button>
                </div>
            }
        } else {
            view! {
                <div>
                    <input
                        type="text"
                        value={username.get()}
                        on:input=move |e| {
                            if let Some(input) = e.target().unwrap().dyn_into::<HtmlInputElement>().ok() {
                                set_username.set(input.value());
                            }
                        }
                    />
                    <button on:click=on_login>"Login"</button>
                </div>
            }
        }
    };

    view! {
        <div>
            {render_auth_state()}
        </div>
    }
}
// view! {
//     <div>
//         {(move || {
//             if auth_state.user.0.get().logged_in {
//                 view! {
//                     <div>
//                         <p>{format!("Welcome, {}", auth_state.user.0.get().username)}</p>
//                         <button on:click=on_logout>"Logout"</button>
//                     </div>
//                 }.into_view()
//             } else {
//                 view! {
//                     <div>
//                         <input
//                             type="text"
//                             value={username.get()}
//                             on:input=move |e| {
//                                 if let Some(input) = e.target().unwrap().dyn_into::<HtmlInputElement>().ok() {
//                                     set_username.set(input.value());
//                                 }
//                             }
//                         />
//                         <button on:click=on_login>"Login"</button>
//                     </div>
//                 }.into_view()
//             }
//         })()}
//     </div>
// }

// let render_auth_state = move || {
//     if auth_state.user.0.get().logged_in {
//         view! {
//             <div>
//                 <p>{format!("Welcome, {}", auth_state.user.0.get().username)}</p>
//                 <button on:click=on_logout>"Logout"</button>
//             </div>
//         }
//     } else {
//         view! {
//             <div>
//                 <input
//                     type="text"
//                     value={username.get()}
//                     on:input=move |e| {
//                         if let Some(input) = e.target().unwrap().dyn_into::<HtmlInputElement>().ok() {
//                             set_username.set(input.value());
//                         }
//                     }
//                 />
//                 <button on:click=on_login>"Login"</button>
//             </div>
//         }
//     }
// };
// }

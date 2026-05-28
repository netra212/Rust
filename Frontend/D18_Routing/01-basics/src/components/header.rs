use leptos::prelude::*;
use leptos_router::{components:A, hooks:use_location};
use leptos_ui::clx;

/**
 * components:A -> This is NOT a regular <a>. It is a Leptos Router's enhanced anchor component that:
 *             -> Prevents full page reloading (intercepts click events).
 *             -> Updates the URL using the History API. 
 *             -> Triggers the client-side routing instead of server requests.
 *             -> Works like React Router's <Link> component.
 * use_location: A hook that gives you reactive access to the current URL.Think of it as a live feed of where the user is in your app.When the URL changes, anything depending on this hook automatically updates.
 * 
 * What is clx ?
 * = It's a utility for creating styled component aliases. Instead of writing <nav class="flex gap-2 p-4"> everywhere, you create a reusable Navbar component that always has those classes.It's like creating styled-components in React, but using Tailwind classes.
 * 
 * Q. When to use clx! ?
 * Use clx! when: we just want a styled wrapper with no logic.
 * 
 * Q. When to use component ?
 * Use a component when: we need state, effects, or complex behavior.
 * 
 * Why <header>? Semantic HTML. It tells browsers and screen readers "this is the header section of the page." It's an HTML5 landmark that improves accessibility.
 */

#[component]
pub fn Header() -> impl IntoView{
    clx! {Navbar, nav, "flex gap-2 p-4"} // Create a Component called Navbar that render a <nav> element with Tailwind classes. such as: flex, gap-2, p-4. or clx! generates this boilerplate.

    view!{
        <header>
            <Navbar>
                <MenuLink href="/">"Home Page"</MenuLink>
                <MenuLink href="/user">"User Page"</MenuLink>
                <MenuLink href="/page/with/path">"Page with Path"</MenuLink>
                <MenuLink href="/page/static/segment">"Page with static segment"</MenuLink>
            </Navbar>
        </header>
    }
}

// #[component]
// pub fn Navbar(children: Children) -> impl IntoView{
//     view!{
//         <nav class="flex gap-2 p-4">
//             {children()}
//         </nav>
//     }
// }

/**
 * 
# Analogy with React.

React:    function MenuLink() { }     ← automatically a component
Leptos:   #[component]                ← must explicitly mark it
          pub fn MenuLink() { }

 */

/**
pub fn MenuLink()
- What it is: A public function named MenuLink.
React equivalent: export function MenuLink() or export default function MenuLink()
 */

 /**
  * #[prop(into)] href: &'static str

    &'static str -> A string reference (borrowed string) that lives for the ENTIRE program

    Q. why 'static ?
        - means we can ONLY pass hardcoded strings. 

    'static Lifetime means: This string is embedded in the compiled binary.It's NOT from user input, NOT from a database, NOT generated at runtime. It's like a constant.

    #[prop(into)] - The Magic Converter
    What it does: Automatically converts whatever you pass into the required type.

    Without #[prop(into)]:
    // You MUST pass exactly &'static str
    <MenuLink href="/home" />  // Only this works

    With #[prop(into)]:
    // You can pass anything that can BECOME &'static str
    <MenuLink href="/home" />                    // &str → &'static str
    <MenuLink href={String::from("/home")} />    // String → &'static str  

    NOTE: Think of #[prop(into)] like this: It's like saying: "I want a &'static str, but I'll accept 
    anything that can TURN INTO a &'static str".

  */


/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                         */
/* ========================================================== */


#[component] //  <-- macros that transforms the regular rust function into a leptos component. 
pub fn MenuLink(#[prop(into)] href: &'static str, children: Children) -> impl IntoView {
    let location = use_location();
    let is_active = Memo::new(move |_| location.pathname.get() == href);

    view! {
        <A class:font-bold=move || is_active.get() href=href>
            <span class="py-2 px-4 rounded-md bg-accent">{children()}</span>
        </A>
    }
}

/**
 * What is a Memo?
    A Memo is a derived reactive value that:
        - Caches its computation
        - Recalculates only when its dependencies change
        - Notifies subscribers when the result changes
    Think of it like a smart cache
        a. If location.pathname hasn't changed, don't re-check if is_active - just return the cached result
        b. If location.pathname changed, recalculate and notify any UI that depends on this
 */



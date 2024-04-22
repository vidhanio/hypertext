#![allow(clippy::useless_vec)]

use hypertext::{Attribute, AttributeNamespace, GlobalAttributes};

#[test]
fn readme() {
    use hypertext::{html_elements, GlobalAttributes, RenderIterator, Renderable};

    let shopping_list = vec!["milk", "eggs", "bread"];

    let shopping_list_maud = hypertext::maud! {
        div {
            h1 { "Shopping List" }
            ul {
                @for (&item, i) in shopping_list.iter().zip(1..) {
                    li.item {
                        input #{ "item-" (i) } type="checkbox";
                        label for={ "item-" (i) } { (item) }
                    }
                }
            }
        }
    }
    .render();

    // or, alternatively:

    let shopping_list_rsx = hypertext::rsx! {
        <div>
            <h1>Shopping List</h1>
            <ul>
                { shopping_list.iter().zip(1..).map(|(&item, i)| hypertext::rsx_move! {
                    <li class="item">
                        <input id=format!("item-{i}") type="checkbox">
                        <label for=format!("item-{i}")>{ item }</label>
                    </li>
                }).render_all() }
            </ul>
        </div>
    }
    .render();

    assert_eq!(shopping_list_maud, shopping_list_rsx);
}

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
trait HtmxAttributes: GlobalAttributes {
    const hx_post: Attribute = Attribute;
    const hx_on: AttributeNamespace = AttributeNamespace;
}

impl<T: GlobalAttributes> HtmxAttributes for T {}

#[test]
fn htmx() {
    use hypertext::{html_elements, Renderable};

    let htmx_maud = hypertext::maud! {
        div {
            form hx-post="/login" hx-on::after-request="this.reset()" {
                input type="text" name="username";
                input type="password" name="password";
                input type="submit" value="Login";
            }
        }
    }
    .render();

    assert_eq!(
        htmx_maud,
        r#"<div><form hx-post="/login" hx-on::after-request="this.reset()"><input type="text" name="username"><input type="password" name="password"><input type="submit" value="Login"></form></div>"#
    );
}

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

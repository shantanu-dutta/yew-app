use gloo_utils::document;
use wasm_bindgen::JsCast;
use web_sys::{Url};
use web_sys::{DragEvent, HtmlImageElement};
use yew::{html, Component, Context, Html};

pub enum Msg {
  Dropped(DragEvent),
  Dragged(DragEvent),
}

pub struct DropImage {
  images: Vec<String>,
}

impl Component for DropImage {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self { images: vec!() }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::Dragged(event) => {
        event.prevent_default();
        false
      },
      Msg::Dropped(event) => {
        event.prevent_default();

        // access DataTransfer api
        let data_transfer = event.data_transfer().expect("Event should have DataTransfer");
        let item_list = data_transfer.items();

        for i in 0..item_list.length() {
          let item = item_list.get(i).expect("Should find an item");

          if item.kind() == "file" {
            let file = item.get_as_file().expect("Should find a file here").unwrap();

            // create img element
            let element = document().create_element("img").unwrap();
            let img = element.dyn_ref::<HtmlImageElement>().expect("Cannot create image element");
            let url = Url::create_object_url_with_blob(&file).expect("Cannot create url");
            img.set_src(&url);
            img.set_width(100);
            img.set_height(100);

            // append it to container div
            if let Some(images) = document().get_element_by_id("images") {
              images.append_child(img).expect("Cannot add photo");
            }

            // update component state
            self.images.push(file.name());
          }
        }

        true
      },
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let link = ctx.link();

    html! {
      <>
        <div class="drop-zone"
            ondragover={link.callback(|e| Msg::Dragged(e))}
            ondrop={link.callback(|e| Msg::Dropped(e))}
        >
          <p>{"Drag your images here"}</p>
        </div>
        <div id="images"></div>
        <div>{ self.images.iter().collect::<Html>() }</div>
      </>
    }
  }
}

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;
use payden_model::*;
use reactive_stores::Store;
use thaw::{ConfigProvider, ToasterProvider};

use crate::common::*;
use crate::constants::*;
use crate::page::*;
use crate::sig;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ConfigProvider>
            <ToasterProvider>
                <Router>
                    <Stylesheet id="leptos" href="/style/output.css"/>
                    <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
                    <Home/>
                </Router>
            </ToasterProvider>
        </ConfigProvider>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let model = Store::new(Model::default());

    // TODO: remove this
    model
        .address()
        .set("84f5946bb3Bf4630Afe6aB94EAC561bD015F67c0".to_string());

    provide_context(model);

    view! {
        <main>
            <div class="
                flex flex-col items-center gap-4
                min-h-screen
                bg-orange-50
            ">
                <Header/>
                <Card>
                    <Routes fallback=PageNotFound>
                        <Route path=path!("") view=sig! { view! { <Redirect path=PATH_DEFAULT/> }}/>
                        <ParentRoute path=path!("/wallet") view=Wallet>
                            <Route path=path!("") view=sig! { view! { <Redirect path=PATH_DEFAULT/> }}/>
                            <Route path=path!("/send") view=PageSend/>
                            <Route path=path!("/receive") view=PageReceive/>
                            <Route path=path!("/faucet") view=PageFaucet/>
                        </ParentRoute>
                    </Routes>
                </Card>
                <Footer/>
            </div>
        </main>
    }
}

#[component]
pub fn Wallet() -> impl IntoView {
    let model = expect_context::<Store<Model>>();

    view! {
        <div class="flex flex-col gap-4">
            <Address address=sig! { model.address().get() }/>
            <Balance balance=sig! {model.balance().get() }/>
            <div class="grid grid-cols-2 gap-x-8 gap-y-4">
                <ButtonNavigateSend
                    on_press=sig! { model.page().set(Page::Send) }
                    active=sig! { model.page().get() == Page::Send }
                />
                <ButtonNavigateReceive
                    on_press=sig! { model.page().set(Page::Receive) }
                    active=sig! { model.page().get() == Page::Receive }
                />
                <ButtonNavigateFaucet
                    on_press=sig! { model.page().set(Page::Faucet) }
                    active=sig! { model.page().get() == Page::Faucet }
                />
                <ButtonNavigateActivity
                    on_press=sig! { model.page().set(Page::Activity) }
                    active=sig! { model.page().get() == Page::Activity }
                />
            </div>
            <Outlet/>
        </div>
    }
}

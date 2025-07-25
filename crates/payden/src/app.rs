use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;
use payden::PLACEHOLDER_ADDRESS;
use payden_model::*;
use thaw::{ConfigProvider, ToasterProvider};

use crate::prelude::*;

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
    let controller = LocalResource::new(move || payden_controller::Controller::new());
    let message_bus = Action::new_local(move |action: &payden_controller::ControllerAction| {
        let action = action.clone();
        async move {
            let controller = controller.await;
            let mut controller = controller.borrow_mut();
            controller.handle(action).await;
        }
    });

    provide_context::<Context>(controller);
    provide_context::<MessageBus>(message_bus);

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
    let context = expect_context::<Context>();

    view! {
        <div class="flex flex-col gap-4">
            <Suspense fallback=PageLoading>
                <Address address=sig! {
                    context.read()
                        .as_ref()
                        .map(|controller| controller.borrow().model.address().get())
                        .unwrap_or(PLACEHOLDER_ADDRESS.to_string())
                }/>
                <Balance balance=sig! {
                    context.read()
                        .as_ref()
                        .map(|controller| controller.borrow().model.balance().get())
                        .unwrap_or_default()
                }/>
                <div class="grid grid-cols-2 gap-x-8 gap-y-4">
                    <ButtonNavigateSend/>
                    <ButtonNavigateReceive/>
                    <ButtonNavigateFaucet/>
                    <ButtonNavigateActivity/>
                </div>
                <Outlet/>
                <p class="text-rose-400 italic text-s text-center font-bold w-full">
                    Do not send mainnet funds to this address
                </p>
            </Suspense>
        </div>
    }
}

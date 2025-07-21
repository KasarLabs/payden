use leptos::prelude::*;

#[component]
fn Icon(width: u32, height: u32, children: Children) -> impl IntoView {
    view! {
        <svg
            width={width}
            height={height}
            viewBox="0 0 25 25"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            {children()}
        </svg>
    }
}

#[component]
pub fn IconBook(#[prop(default = 24)] size: u32) -> impl IntoView {
    view! {
        <Icon
            width={size}
            height={size}
        >
            <path d="M4 19V5C4 3.89543 4.89543 3 6 3H19.4C19.7314 3 20 3.26863 20 3.6V16.7143" stroke-linecap="round"/><path d="M15 17V22L17.5 20.4L20 22V17" stroke-linecap="round" stroke-linejoin="round"/><path d="M6 17L20 17" stroke-linecap="round"/><path d="M6 17C4.89543 17 4 17.8954 4 19C4 20.1046 4.89543 21 6 21H11.5" stroke-linecap="round" stroke-linejoin="round"/>
        </Icon>
    }
}

#[component]
pub fn IconClose(#[prop(default = 24)] size: u32) -> impl IntoView {
    view! {
        <Icon
            width={size}
            height={size}
        >
            <path d="M6.75827 17.2426L12.0009 12M17.2435 6.75736L12.0009 12M12.0009 12L6.75827 6.75736M12.0009 12L17.2435 17.2426" stroke-linecap="round" stroke-linejoin="round"></path>
        </Icon>
    }
}

#[component]
pub fn IconCopy(#[prop(default = 24)] size: u32) -> impl IntoView {
    view! {
        <Icon
            width={size}
            height={size}
        >
            <path d="M19.4 20H9.6C9.26863 20 9 19.7314 9 19.4V9.6C9 9.26863 9.26863 9 9.6 9H19.4C19.7314 9 20 9.26863 20 9.6V19.4C20 19.7314 19.7314 20 19.4 20Z" stroke-linecap="round" stroke-linejoin="round"/><path d="M15 9V4.6C15 4.26863 14.7314 4 14.4 4H4.6C4.26863 4 4 4.26863 4 4.6V14.4C4 14.7314 4.26863 15 4.6 15H9" stroke-linecap="round" stroke-linejoin="round"/>
        </Icon>
    }
}

#[component]
pub fn IconDrop(#[prop(default = 24)] size: u32) -> impl IntoView {
    view! {
        <Icon
            width={size}
            height={size}
        >
            <path d="M20 14C20 9.58172 12 2 12 2C12 2 4 9.58172 4 14C4 18.4183 7.58172 22 12 22C16.4183 22 20 18.4183 20 14Z"/>
        </Icon>
    }
}

#[component]
pub fn IconGithubCircle(#[prop(default = 24)] size: u32) -> impl IntoView {
    view! {
        <Icon
            width={size}
            height={size}
        >
            <path d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z" stroke-linecap="round" stroke-linejoin="round"/><path d="M14.3333 19V17.137C14.3583 16.8275 14.3154 16.5163 14.2073 16.2242C14.0993 15.9321 13.9286 15.6657 13.7067 15.4428C15.8 15.2156 18 14.4431 18 10.8989C17.9998 9.99256 17.6418 9.12101 17 8.46461C17.3039 7.67171 17.2824 6.79528 16.94 6.01739C16.94 6.01739 16.1533 5.7902 14.3333 6.97811C12.8053 6.57488 11.1947 6.57488 9.66666 6.97811C7.84666 5.7902 7.05999 6.01739 7.05999 6.01739C6.71757 6.79528 6.69609 7.67171 6.99999 8.46461C6.35341 9.12588 5.99501 10.0053 5.99999 10.9183C5.99999 14.4366 8.19999 15.2091 10.2933 15.4622C10.074 15.6829 9.90483 15.9461 9.79686 16.2347C9.68889 16.5232 9.64453 16.8306 9.66666 17.137V19" stroke-linecap="round" stroke-linejoin="round"/><path d="M9.66667 17.7018C7.66667 18.3335 6 17.7018 5 15.7544" stroke-linecap="round" stroke-linejoin="round"/>
        </Icon>
    }
}

#[component]
pub fn IconGithub(#[prop(default = 24)] size: u32) -> impl IntoView {
    view! {
        <Icon
            width={size}
            height={size}
        >
            <path d="M16 22.0268V19.1568C16.0375 18.68 15.9731 18.2006 15.811 17.7506C15.6489 17.3006 15.3929 16.8902 15.06 16.5468C18.2 16.1968 21.5 15.0068 21.5 9.54679C21.4997 8.15062 20.9627 6.80799 20 5.79679C20.4558 4.5753 20.4236 3.22514 19.91 2.02679C19.91 2.02679 18.73 1.67679 16 3.50679C13.708 2.88561 11.292 2.88561 8.99999 3.50679C6.26999 1.67679 5.08999 2.02679 5.08999 2.02679C4.57636 3.22514 4.54413 4.5753 4.99999 5.79679C4.03011 6.81549 3.49251 8.17026 3.49999 9.57679C3.49999 14.9968 6.79998 16.1868 9.93998 16.5768C9.61098 16.9168 9.35725 17.3222 9.19529 17.7667C9.03334 18.2112 8.96679 18.6849 8.99999 19.1568V22.0268" stroke-linecap="round" stroke-linejoin="round"/><path d="M9 20.0267C6 20.9999 3.5 20.0267 2 17.0267" stroke-linecap="round" stroke-linejoin="round"/>
        </Icon>
    }
}

#[component]
pub fn IconMiden(#[prop(default = 24)] size: u32) -> impl IntoView {
    view! {
        <Icon
            width={size}
            height={size}
        >
            <path d="M12.4531 7.4L13.9496 9.04722C14.0774 9.1875 14.3108 9.09722 14.3108 8.90833V4.96389C14.3108 4.90887 14.3325 4.85609 14.3713 4.81706C14.4101 4.77803 14.4627 4.75592 14.5177 4.75556H17.2677C18.4746 4.75556 19.5927 5.38611 20.2114 6.41528L20.7545 7.31806C20.7906 7.37792 20.8415 7.42745 20.9023 7.46188C20.9632 7.49631 21.0318 7.51446 21.1017 7.51458H21.1226C21.2302 7.5144 21.3333 7.47152 21.4094 7.39535C21.4854 7.31918 21.5281 7.21596 21.5281 7.10833V3.00556C21.5281 2.89793 21.4854 2.79471 21.4094 2.71854C21.3333 2.64237 21.2302 2.59949 21.1226 2.59931H14.7253C14.6154 2.59931 14.5099 2.55563 14.4322 2.47788C14.3544 2.40013 14.3108 2.29468 14.3108 2.18472V0.311111C14.3108 0.139583 14.1719 0 13.9996 0H12.5489C12.3774 0 12.2378 0.138889 12.2378 0.311111V2.18472C12.2378 2.41389 12.0524 2.6 11.8233 2.6H3.87881C3.7713 2.6 3.66818 2.64261 3.59204 2.7185C3.51589 2.79438 3.47292 2.89736 3.47256 3.00486V3.97292C3.47256 4.07708 3.51284 4.17708 3.58436 4.25278L12.4989 12.2208C12.5702 12.296 12.6099 12.3957 12.6099 12.4993C12.6099 12.6029 12.5702 12.7026 12.4989 12.7778L3.58367 20.7465C3.51196 20.8221 3.47194 20.9222 3.47186 21.0264V21.9944C3.47186 22.2187 3.65381 22.4 3.87811 22.4H11.8226C12.0517 22.4 12.2371 22.5861 12.2371 22.8153V24.6889C12.2371 24.8604 12.376 25 12.5483 25H13.9989C14.1705 25 14.3101 24.8611 14.3101 24.6889V22.8153C14.3101 22.5861 14.4955 22.4007 14.7239 22.4007H21.1212C21.1747 22.4008 21.2277 22.3903 21.2772 22.3698C21.3266 22.3494 21.3716 22.3193 21.4094 22.2815C21.4472 22.2436 21.4771 22.1986 21.4975 22.1491C21.5179 22.0996 21.5283 22.0466 21.5281 21.9931V17.891C21.5281 17.7832 21.4853 17.6799 21.4091 17.6037C21.3329 17.5275 21.2296 17.4847 21.1219 17.4847H21.1024C21.0323 17.4846 20.9633 17.5026 20.9022 17.5371C20.8411 17.5715 20.79 17.6212 20.7538 17.6812L20.2114 18.584C19.9056 19.0909 19.4739 19.5102 18.9582 19.801C18.4425 20.0918 17.8604 20.2443 17.2684 20.2437H14.5177C14.4904 20.2437 14.4634 20.2382 14.4383 20.2277C14.4131 20.2172 14.3903 20.2018 14.3711 20.1824C14.3519 20.1631 14.3366 20.1402 14.3263 20.1149C14.3159 20.0897 14.3107 20.0627 14.3108 20.0354V16.091C14.3109 16.049 14.2983 16.008 14.2746 15.9733C14.2509 15.9387 14.2173 15.9121 14.1781 15.897C14.1389 15.8819 14.0961 15.8791 14.0553 15.889C14.0145 15.8988 13.9776 15.9208 13.9496 15.9521L12.4531 17.5993C12.3142 17.7521 12.2378 17.9507 12.2378 18.1576V20.0361C12.2378 20.091 12.216 20.1436 12.1772 20.1824C12.1384 20.2213 12.0858 20.2431 12.0309 20.2431H6.93089C6.91929 20.2434 6.9078 20.2407 6.89757 20.2352C6.88734 20.2297 6.87872 20.2216 6.87256 20.2118C6.86573 20.2002 6.86267 20.1867 6.86379 20.1733C6.86491 20.1599 6.87015 20.1471 6.87881 20.1368L14.8253 12.6694L14.8267 12.6674C14.8653 12.6197 14.8864 12.5603 14.8864 12.499C14.8864 12.4376 14.8653 12.3782 14.8267 12.3306L14.8253 12.3292L6.87881 4.8625C6.87015 4.85217 6.86491 4.83942 6.86379 4.82599C6.86267 4.81256 6.86573 4.79912 6.87256 4.7875C6.87872 4.77766 6.88734 4.76961 6.89757 4.76413C6.9078 4.75864 6.91929 4.75593 6.93089 4.75625H12.0309C12.1448 4.75625 12.2378 4.84861 12.2378 4.96319V6.84167C12.2378 7.04792 12.3142 7.24722 12.4531 7.4Z"/>
        </Icon>
    }
}

#[component]
pub fn IconPulse(#[prop(default = 24)] size: u32) -> impl IntoView {
    view! {
        <Icon
            width={size}
            height={size}
        >
            <path d="M23 12.75H18.3333L14.8333 20.9166L10.1667 4.58331L6.66667 12.75H2" stroke-linecap="round" stroke-linejoin="round"/>
        </Icon>
    }
}

#[component]
pub fn IconQr(#[prop(default = 24)] size: u32) -> impl IntoView {
    view! {
        <Icon
            width={size}
            height={size}
        >
            <path d="M15 12L15 15" stroke-linecap="round" stroke-linejoin="round"></path><path d="M12 3V6" stroke-linecap="round" stroke-linejoin="round"></path><path d="M18 12L18 15" stroke-linecap="round" stroke-linejoin="round"></path><path d="M12 18L21 18" stroke-linecap="round" stroke-linejoin="round"></path><path d="M18 21H21" stroke-linecap="round" stroke-linejoin="round"></path><path d="M6 12H9" stroke-linecap="round" stroke-linejoin="round"></path><path d="M6 6.01111L6.01 6" stroke-linecap="round" stroke-linejoin="round"></path><path d="M12 12.0111L12.01 12" stroke-linecap="round" stroke-linejoin="round"></path><path d="M3 12.0111L3.01 12" stroke-linecap="round" stroke-linejoin="round"></path><path d="M12 9.01111L12.01 9" stroke-linecap="round" stroke-linejoin="round"></path><path d="M12 15.0111L12.01 15" stroke-linecap="round" stroke-linejoin="round"></path><path d="M15 21.0111L15.01 21" stroke-linecap="round" stroke-linejoin="round"></path><path d="M12 21.0111L12.01 21" stroke-linecap="round" stroke-linejoin="round"></path><path d="M21 12.0111L21.01 12" stroke-linecap="round" stroke-linejoin="round"></path><path d="M21 15.0111L21.01 15" stroke-linecap="round" stroke-linejoin="round"></path><path d="M18 6.01111L18.01 6" stroke-linecap="round" stroke-linejoin="round"></path><path d="M9 3.6V8.4C9 8.73137 8.73137 9 8.4 9H3.6C3.26863 9 3 8.73137 3 8.4V3.6C3 3.26863 3.26863 3 3.6 3H8.4C8.73137 3 9 3.26863 9 3.6Z" stroke-linecap="round" stroke-linejoin="round"></path><path d="M21 3.6V8.4C21 8.73137 20.7314 9 20.4 9H15.6C15.2686 9 15 8.73137 15 8.4V3.6C15 3.26863 15.2686 3 15.6 3H20.4C20.7314 3 21 3.26863 21 3.6Z" stroke-linecap="round" stroke-linejoin="round"></path><path d="M6 18.0111L6.01 18" stroke-linecap="round" stroke-linejoin="round"></path><path d="M9 15.6V20.4C9 20.7314 8.73137 21 8.4 21H3.6C3.26863 21 3 20.7314 3 20.4V15.6C3 15.2686 3.26863 15 3.6 15H8.4C8.73137 15 9 15.2686 9 15.6Z" stroke-linecap="round" stroke-linejoin="round"></path>
        </Icon>
    }
}

#[component]
pub fn IconSend(#[prop(default = 24)] size: u32) -> impl IntoView {
    view! {
        <Icon
            width={size}
            height={size}
        >
            <path fill-rule="evenodd" clip-rule="evenodd" d="M1.84647 7.15123C1.54566 7.21608 1.31498 7.45811 1.26464 7.7617C1.2143 8.06528 1.35452 8.36881 1.6183 8.52729L8.13474 12.4421L14.3544 8.08705C14.6938 7.84947 15.1614 7.93193 15.399 8.27123C15.6366 8.61054 15.5541 9.0782 15.2148 9.31578L8.99537 13.6707L10.4455 21.1339C10.5042 21.436 10.7415 21.6715 11.044 21.7281C11.3465 21.7846 11.6528 21.6506 11.8166 21.3901L22.7919 3.93893C22.9526 3.68349 22.9445 3.35665 22.7714 3.10947C22.5983 2.86228 22.294 2.7429 21.999 2.80649L1.84647 7.15123Z"></path>
        </Icon>
    }
}

#[component]
pub fn IconSettings(#[prop(default = 24)] size: u32) -> impl IntoView {
    view! {
        <Icon
            width={size}
            height={size}
        >
            <path d="M12 15C13.6569 15 15 13.6569 15 12C15 10.3431 13.6569 9 12 9C10.3431 9 9 10.3431 9 12C9 13.6569 10.3431 15 12 15Z" stroke-linecap="round" stroke-linejoin="round"></path><path d="M19.6224 10.3954L18.5247 7.7448L20 6L18 4L16.2647 5.48295L13.5578 4.36974L12.9353 2H10.981L10.3491 4.40113L7.70441 5.51596L6 4L4 6L5.45337 7.78885L4.3725 10.4463L2 11V13L4.40111 13.6555L5.51575 16.2997L4 18L6 20L7.79116 18.5403L10.397 19.6123L11 22H13L13.6045 19.6132L16.2551 18.5155C16.6969 18.8313 18 20 18 20L20 18L18.5159 16.2494L19.6139 13.598L21.9999 12.9772L22 11L19.6224 10.3954Z" stroke-linecap="round" stroke-linejoin="round"></path>
        </Icon>
    }
}
#[component]
pub fn IconTelegram(#[prop(default = 24)] size: u32) -> impl IntoView {
    view! {
        <Icon
            width={size}
            height={size}
        >
            <path d="M21 5L2 12.5L9 13.5M21 5L18.5 20L9 13.5M21 5L9 13.5M9 13.5V19L12.2488 15.7229" stroke-linecap="round" stroke-linejoin="round"></path>
        </Icon>
    }
}

#[component]
pub fn IconTwitter(#[prop(default = 24)] size: u32) -> impl IntoView {
    view! {
        <Icon
            width={size}
            height={size}
        >
            <path d="M16.8198 20.7684L3.75317 3.96836C3.44664 3.57425 3.72749 3 4.22678 3H6.70655C6.8917 3 7.06649 3.08548 7.18016 3.23164L20.2468 20.0316C20.5534 20.4258 20.2725 21 19.7732 21H17.2935C17.1083 21 16.9335 20.9145 16.8198 20.7684Z"></path><path d="M20 3L4 21" stroke-linecap="round"></path>
        </Icon>
    }
}

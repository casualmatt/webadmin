use std::sync::Arc;

use leptos::*;
use leptos_router::*;

use crate::{
    core::http::{self, HttpRequest},
    pages::{
        directory::{List, Principal, Type},
        maybe_plural,
    },
};

const PAGE_SIZE: u64 = 1;

#[component]
pub fn AccountList() -> impl IntoView {
    let query = use_query_map();
    let page = move || {
        query
            .with(|q| q.get("page").and_then(|page| page.parse::<u64>().ok()))
            .unwrap_or(1)
    };
    let filter = move || {
        query.with(|q| {
            q.get("filter").and_then(|s| {
                let s = s.trim();
                if !s.is_empty() {
                    Some(s.to_string())
                } else {
                    None
                }
            })
        })
    };

    let principals = create_resource(
        move || (page(), filter()),
        move |(page, filter)| async move {
            let account_names = HttpRequest::get("https://127.0.0.1/api/principal?")
                .with_header("Authorization", "Basic YWRtaW46c2VjcmV0")
                .with_parameter("page", page.to_string())
                .with_parameter("limit", PAGE_SIZE.to_string())
                .with_parameter("type", "individual")
                .with_optional_parameter("filter", filter)
                .send::<List<String>>()
                .await?;
            let mut items = Vec::with_capacity(account_names.items.len());

            for name in account_names.items {
                items.push(
                    HttpRequest::get(&format!("https://127.0.0.1/api/principal/{}", name))
                        .with_header("Authorization", "Basic YWRtaW46c2VjcmV0")
                        .send::<Principal>()
                        .await?,
                );
            }

            Ok(Arc::new(List {
                items,
                total: account_names.total,
            }))
        },
    );
    let (pending, set_pending) = create_signal(false);

    let hide_more_link = move || {
        let total_items = principals
            .get()
            .unwrap_or_else(|| Ok(Arc::new(List::default())))
            .unwrap_or_default()
            .total;

        let total_pages = (total_items as f64 / PAGE_SIZE as f64).ceil() as u64;

        page() >= total_pages || pending()
    };

    view! {
        <div class="max-w-[85rem] px-4 py-10 sm:px-6 lg:px-8 lg:py-14 mx-auto">
            <div class="flex flex-col">
                <div class="-m-1.5 overflow-x-auto">
                    <div class="p-1.5 min-w-full inline-block align-middle">
                        <div class="bg-white border border-gray-200 rounded-xl shadow-sm overflow-hidden dark:bg-slate-900 dark:border-gray-700">
                            <div class="px-6 py-4 grid gap-3 md:flex md:justify-between md:items-center border-b border-gray-200 dark:border-gray-700">
                                <div>
                                    <h2 class="text-xl font-semibold text-gray-800 dark:text-gray-200">
                                        Users
                                    </h2>
                                    <p class="text-sm text-gray-600 dark:text-gray-400">
                                        Add users, edit and more.
                                    </p>
                                </div>

                                <div>
                                    <div class="inline-flex gap-x-2">
                                        <div class="sm:col-span-1">
                                            <label
                                                for="hs-as-table-product-review-search"
                                                class="sr-only"
                                            >
                                                Search
                                            </label>
                                            <div class="relative">
                                                <input
                                                    type="text"
                                                    id="hs-as-table-product-review-search"
                                                    name="hs-as-table-product-review-search"
                                                    class="py-2 px-3 ps-11 block w-full border-gray-200 rounded-lg text-sm focus:border-blue-500 focus:ring-blue-500 disabled:opacity-50 disabled:pointer-events-none dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400 dark:focus:ring-gray-600"
                                                    placeholder="Search"
                                                />
                                                <div class="absolute inset-y-0 start-0 flex items-center pointer-events-none ps-4">
                                                    <svg
                                                        class="size-4 text-gray-400"
                                                        xmlns="http://www.w3.org/2000/svg"
                                                        width="16"
                                                        height="16"
                                                        fill="currentColor"
                                                        viewBox="0 0 16 16"
                                                    >
                                                        <path d="M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001c.03.04.062.078.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1.007 1.007 0 0 0-.115-.1zM12 6.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0z"></path>
                                                    </svg>
                                                </div>
                                            </div>
                                        </div>

                                        <a
                                            class="py-2 px-3 inline-flex items-center gap-x-2 text-sm font-medium rounded-lg border border-gray-200 bg-white text-red-500 shadow-sm hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none dark:bg-slate-900 dark:border-gray-700 dark:hover:bg-gray-800 dark:focus:outline-none dark:focus:ring-1 dark:focus:ring-gray-600"
                                            href="#"
                                        >
                                            <svg
                                                class="flex-shrink-0 size-4"
                                                xmlns="http://www.w3.org/2000/svg"
                                                width="24"
                                                height="24"
                                                viewBox="0 0 24 24"
                                                fill="none"
                                                stroke="currentColor"
                                                stroke-width="2"
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                            >
                                                <path d="M3 6h18"></path>
                                                <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"></path>
                                                <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"></path>
                                                <line x1="10" x2="10" y1="11" y2="17"></line>
                                                <line x1="14" x2="14" y1="11" y2="17"></line>
                                            </svg>
                                            Delete (2)
                                        </a>

                                        <a
                                            class="py-2 px-3 inline-flex items-center gap-x-2 text-sm font-semibold rounded-lg border border-transparent bg-blue-600 text-white hover:bg-blue-700 disabled:opacity-50 disabled:pointer-events-none dark:focus:outline-none dark:focus:ring-1 dark:focus:ring-gray-600"
                                            href="#"
                                        >
                                            <svg
                                                class="flex-shrink-0 size-3"
                                                xmlns="http://www.w3.org/2000/svg"
                                                width="16"
                                                height="16"
                                                viewBox="0 0 16 16"
                                                fill="none"
                                            >
                                                <path
                                                    d="M2.63452 7.50001L13.6345 7.5M8.13452 13V2"
                                                    stroke="currentColor"
                                                    stroke-width="2"
                                                    stroke-linecap="round"
                                                ></path>
                                            </svg>
                                            Add user
                                        </a>
                                    </div>
                                </div>
                            </div>

                            <Transition fallback=move || view! { <p>"Loading..."</p> } set_pending>
                                {move || match principals.get() {
                                    None => None,
                                    Some(Err(http::Error::Unauthorized)) => {
                                        Some(view! { <p>"Unauthorized."</p> }.into_any())
                                    }
                                    Some(Err(err)) => {
                                        Some(
                                            view! {
                                                <p>"Error loading principals." {format!("{err:?}")}</p>
                                            }
                                                .into_any(),
                                        )
                                    }
                                    Some(Ok(principals)) if !principals.items.is_empty() => {
                                        Some(
                                            view! {
                                                <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                                                    <thead class="bg-gray-50 dark:bg-slate-800">
                                                        <tr>
                                                            <th scope="col" class="ps-6 py-3 text-start">
                                                                <label for="hs-at-with-checkboxes-main" class="flex">
                                                                    <input
                                                                        type="checkbox"
                                                                        class="shrink-0 border-gray-300 rounded text-blue-600 focus:ring-blue-500 disabled:opacity-50 disabled:pointer-events-none dark:bg-slate-900 dark:border-gray-600 dark:checked:bg-blue-500 dark:checked:border-blue-500 dark:focus:ring-offset-gray-800"
                                                                        id="hs-at-with-checkboxes-main"
                                                                    />
                                                                    <span class="sr-only">Checkbox</span>
                                                                </label>
                                                            </th>

                                                            <th
                                                                scope="col"
                                                                class="ps-6 lg:ps-3 xl:ps-0 pe-6 py-3 text-start"
                                                            >
                                                                <div class="flex items-center gap-x-2">
                                                                    <span class="text-xs font-semibold uppercase tracking-wide text-gray-800 dark:text-gray-200">
                                                                        Name
                                                                    </span>
                                                                </div>
                                                            </th>

                                                            <th scope="col" class="px-6 py-3 text-start">
                                                                <div class="flex items-center gap-x-2">
                                                                    <span class="text-xs font-semibold uppercase tracking-wide text-gray-800 dark:text-gray-200">
                                                                        E-mail
                                                                    </span>
                                                                </div>
                                                            </th>

                                                            <th scope="col" class="px-6 py-3 text-start">
                                                                <div class="flex items-center gap-x-2">
                                                                    <span class="text-xs font-semibold uppercase tracking-wide text-gray-800 dark:text-gray-200">
                                                                        Type
                                                                    </span>
                                                                </div>
                                                            </th>

                                                            <th scope="col" class="px-6 py-3 text-start">
                                                                <div class="flex items-center gap-x-2">
                                                                    <span class="text-xs font-semibold uppercase tracking-wide text-gray-800 dark:text-gray-200">
                                                                        Quota
                                                                    </span>
                                                                </div>
                                                            </th>

                                                            <th scope="col" class="px-6 py-3 text-start">
                                                                <div class="flex items-center gap-x-2">
                                                                    <span class="text-xs font-semibold uppercase tracking-wide text-gray-800 dark:text-gray-200">
                                                                        Member of
                                                                    </span>
                                                                </div>
                                                            </th>

                                                            <th scope="col" class="px-6 py-3 text-end"></th>
                                                        </tr>
                                                    </thead>

                                                    <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
                                                        <For
                                                            each=move || principals.items.clone()
                                                            key=|principal| principal.id
                                                            let:principal
                                                        >
                                                            <AccountItem principal/>
                                                        </For>

                                                    </tbody>
                                                </table>
                                            }
                                                .into_any(),
                                        )
                                    }
                                    Some(Ok(_)) => Some(view! { <p>"No results."</p> }.into_any()),
                                }}

                            </Transition>

                            <div class="px-6 py-4 grid gap-3 md:flex md:justify-between md:items-center border-t border-gray-200 dark:border-gray-700">
                                <Suspense>
                                    {move || {
                                        principals
                                            .get()
                                            .map(|result| {
                                                match result {
                                                    Ok(result) if result.total > 0 => {
                                                        let total_pages = (result.total as f64 / PAGE_SIZE as f64)
                                                            .ceil() as u64;
                                                        let cur_page = page();
                                                        let navigate = use_navigate();
                                                        view! {
                                                            <div class="inline-flex items-center gap-x-2">
                                                                <p class="text-sm text-gray-600 dark:text-gray-400">
                                                                    Showing:
                                                                </p>
                                                                <div class="max-w-sm space-y-3">
                                                                    <select
                                                                        class="py-2 px-3 pe-9 block w-full border-gray-200 rounded-lg text-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400"
                                                                        on:change=move |ev| {
                                                                            let new_value = event_target_value(&ev);
                                                                            navigate(
                                                                                &format!("/manage/accounts?page={new_value}"),
                                                                                Default::default(),
                                                                            );
                                                                        }
                                                                    >

                                                                        <For
                                                                            each=move || (1..=total_pages)
                                                                            key=|page| *page
                                                                            let:page
                                                                        >
                                                                            <option selected=move || cur_page == page>{page}</option>

                                                                        </For>

                                                                    </select>
                                                                </div>
                                                                <p class="text-sm text-gray-600 dark:text-gray-400">
                                                                    of {total_pages}
                                                                </p>
                                                            </div>
                                                        }
                                                            .into_any()
                                                    }
                                                    _ => {
                                                        view! {
                                                            <div class="inline-flex items-center gap-x-2">
                                                                <p class="text-sm text-gray-600 dark:text-gray-400">
                                                                    No results.
                                                                </p>
                                                            </div>
                                                        }
                                                            .into_any()
                                                    }
                                                }
                                            })
                                    }}

                                </Suspense>

                                <div>
                                    <div class="inline-flex gap-x-2">
                                        {move || {
                                            if page() > 1 {
                                                view! {
                                                    <a
                                                        href=move || format!("/manage/accounts?page={}", page() - 1)
                                                        class="py-2 px-3 inline-flex items-center gap-x-2 text-sm font-medium rounded-lg border border-gray-200 bg-white text-gray-800 shadow-sm hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none dark:bg-slate-900 dark:border-gray-700 dark:text-white dark:hover:bg-gray-800 dark:focus:outline-none dark:focus:ring-1 dark:focus:ring-gray-600"
                                                    >

                                                        <svg
                                                            class="flex-shrink-0 size-4"
                                                            xmlns="http://www.w3.org/2000/svg"
                                                            width="24"
                                                            height="24"
                                                            viewBox="0 0 24 24"
                                                            fill="none"
                                                            stroke="currentColor"
                                                            stroke-width="2"
                                                            stroke-linecap="round"
                                                            stroke-linejoin="round"
                                                        >
                                                            <path d="m15 18-6-6 6-6"></path>
                                                        </svg>
                                                        Prev
                                                    </a>
                                                }
                                                    .into_any()
                                            } else {
                                                view! {
                                                    <button
                                                        type="button"
                                                        class="py-2 px-3 inline-flex items-center gap-x-2 text-sm font-medium rounded-lg border border-gray-200 bg-white text-gray-800 shadow-sm hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none dark:bg-slate-900 dark:border-gray-700 dark:text-white dark:hover:bg-gray-800 dark:focus:outline-none dark:focus:ring-1 dark:focus:ring-gray-600"
                                                        disabled
                                                    >

                                                        <svg
                                                            class="flex-shrink-0 size-4"
                                                            xmlns="http://www.w3.org/2000/svg"
                                                            width="24"
                                                            height="24"
                                                            viewBox="0 0 24 24"
                                                            fill="none"
                                                            stroke="currentColor"
                                                            stroke-width="2"
                                                            stroke-linecap="round"
                                                            stroke-linejoin="round"
                                                        >
                                                            <path d="m15 18-6-6 6-6"></path>
                                                        </svg>
                                                        Prev
                                                    </button>
                                                }
                                                    .into_any()
                                            }
                                        }}
                                        <Suspense>

                                            {move || {
                                                if !hide_more_link() {
                                                    view! {
                                                        <a
                                                            href=move || format!("/manage/accounts?page={}", page() + 1)
                                                            class="py-2 px-3 inline-flex items-center gap-x-2 text-sm font-medium rounded-lg border border-gray-200 bg-white text-gray-800 shadow-sm hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none dark:bg-slate-900 dark:border-gray-700 dark:text-white dark:hover:bg-gray-800 dark:focus:outline-none dark:focus:ring-1 dark:focus:ring-gray-600"
                                                        >
                                                            Next
                                                            <svg
                                                                class="flex-shrink-0 size-4"
                                                                xmlns="http://www.w3.org/2000/svg"
                                                                width="24"
                                                                height="24"
                                                                viewBox="0 0 24 24"
                                                                fill="none"
                                                                stroke="currentColor"
                                                                stroke-width="2"
                                                                stroke-linecap="round"
                                                                stroke-linejoin="round"
                                                            >
                                                                <path d="m9 18 6-6-6-6"></path>
                                                            </svg>
                                                        </a>
                                                    }
                                                        .into_any()
                                                } else {
                                                    view! {
                                                        <button
                                                            type="button"
                                                            class="invisible py-2 px-3 inline-flex items-center gap-x-2 text-sm font-medium rounded-lg border border-gray-200 bg-white text-gray-800 shadow-sm hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none dark:bg-slate-900 dark:border-gray-700 dark:text-white dark:hover:bg-gray-800 dark:focus:outline-none dark:focus:ring-1 dark:focus:ring-gray-600"
                                                        >

                                                            Next
                                                            <svg
                                                                class="flex-shrink-0 size-4"
                                                                xmlns="http://www.w3.org/2000/svg"
                                                                width="24"
                                                                height="24"
                                                                viewBox="0 0 24 24"
                                                                fill="none"
                                                                stroke="currentColor"
                                                                stroke-width="2"
                                                                stroke-linecap="round"
                                                                stroke-linejoin="round"
                                                            >
                                                                <path d="m9 18 6-6-6-6"></path>
                                                            </svg>
                                                        </button>
                                                    }
                                                        .into_any()
                                                }
                                            }}

                                        </Suspense>

                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn AccountItem(principal: Principal) -> impl IntoView {
    let display_name = principal
        .description
        .as_deref()
        .or(principal.name.as_deref())
        .unwrap_or("Unknown");
    let pct_quota = match (principal.quota, principal.used_quota) {
        (Some(quota), Some(used_quota)) if quota > 0 => format!(
            "{}%",
            (used_quota as f64 / quota as f64 * 100.0).round() as u8
        ),
        _ => "N/A".to_string(),
    };

    view! {
        <tr>
            <td class="size-px whitespace-nowrap">
                <div class="ps-6 py-3">
                    <label for="hs-at-with-checkboxes-1" class="flex">
                        <input
                            type="checkbox"
                            class="shrink-0 border-gray-300 rounded text-blue-600 focus:ring-blue-500 disabled:opacity-50 disabled:pointer-events-none dark:bg-slate-900 dark:border-gray-600 dark:checked:bg-blue-500 dark:checked:border-blue-500 dark:focus:ring-offset-gray-800"
                            id="hs-at-with-checkboxes-1"
                        />
                        <span class="sr-only">Checkbox</span>
                    </label>
                </div>
            </td>
            <td class="size-px whitespace-nowrap">
                <div class="ps-6 lg:ps-3 xl:ps-0 pe-6 py-3">
                    <div class="flex items-center gap-x-3">
                        <span class="inline-flex items-center justify-center size-[38px] rounded-full bg-gray-300 dark:bg-gray-700">
                            <span class="font-medium text-gray-800 leading-none dark:text-gray-200">
                                {display_name
                                    .chars()
                                    .next()
                                    .and_then(|ch| ch.to_uppercase().next())
                                    .unwrap_or_default()}
                            </span>
                        </span>
                        <div class="grow">
                            <span class="block text-sm font-semibold text-gray-800 dark:text-gray-200">
                                {display_name.to_string()}
                            </span>
                            <span class="block text-sm text-gray-500">
                                {principal.name.as_deref().unwrap_or("unknown").to_string()}
                            </span>
                        </div>
                    </div>
                </div>
            </td>
            <td class="h-px w-72 whitespace-nowrap">
                <div class="px-6 py-3">
                    <span class="block text-sm font-semibold text-gray-800 dark:text-gray-200">
                        {principal.emails.first().cloned().unwrap_or_default()}
                    </span>
                    <span class="block text-sm text-gray-500">
                        {maybe_plural(principal.emails.len(), "address", "addresses")}
                    </span>
                </div>
            </td>
            <td class="size-px whitespace-nowrap">
                <div class="px-6 py-3">
                    {if matches!(principal.typ, Some(Type::Superuser)) {
                        view! {
                            <span class="py-1 px-1.5 inline-flex items-center gap-x-1 text-xs font-medium bg-yellow-100 text-yellow-800 rounded-full dark:bg-yellow-500/10 dark:text-yellow-500">
                                Admin
                            </span>
                        }
                    } else {
                        view! {
                            <span class="py-1 px-1.5 inline-flex items-center gap-x-1 text-xs font-medium bg-teal-100 text-teal-800 rounded-full dark:bg-teal-500/10 dark:text-teal-500">
                                Individual
                            </span>
                        }
                    }}

                </div>
            </td>
            <td class="size-px whitespace-nowrap">
                <div class="px-6 py-3">
                    <span class="text-sm text-gray-500">{pct_quota}</span>
                </div>
            </td>
            <td class="size-px whitespace-nowrap">
                <div class="px-6 py-3">
                    <span class="text-sm text-gray-500">
                        {maybe_plural(principal.member_of.len(), "group", "groups")}
                    </span>
                </div>
            </td>
            <td class="size-px whitespace-nowrap">
                <div class="px-6 py-1.5">
                    <a
                        class="inline-flex items-center gap-x-1 text-sm text-blue-600 decoration-2 hover:underline font-medium dark:focus:outline-none dark:focus:ring-1 dark:focus:ring-gray-600"
                        href="#"
                    >
                        Edit
                    </a>
                </div>
            </td>
        </tr>
    }
}
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PaginationProperties {
    pub total_pages: Option<usize>,
    pub on_change: Callback<usize>,
    #[prop_or(1)]
    pub edge_page_count: usize,
    #[prop_or(1)]
    pub sibling_page_count: usize,
}

#[derive(Debug)]
pub enum PaginationMessage {
    Next,
    Prev,
    First,
    Last,
    Set(usize),
}

#[derive(Debug)]
pub struct Pagination {
    total_pages: usize,
    current_page: usize,
}

impl Pagination {
    pub fn set_page(&mut self, page: usize) -> bool {
        // NOTE: Relocate the page to be in range
        let page = if page <= 0 {
            1
        } else if page > self.total_pages {
            self.total_pages
        } else {
            page
        };

        if page != self.current_page {
            self.current_page = page;
            true
        } else {
            false
        }
    }

    pub fn next_page(&mut self) -> bool {
        if self.current_page < self.total_pages {
            self.current_page += 1;
            true
        } else {
            false
        }
    }

    pub fn prev_page(&mut self) -> bool {
        if self.current_page > 1 {
            self.current_page -= 1;
            true
        } else {
            false
        }
    }

    pub fn first_page(&mut self) -> bool {
        self.set_page(1)
    }

    pub fn last_page(&mut self) -> bool {
        self.set_page(self.total_pages)
    }
}

impl Component for Pagination {
    type Message = PaginationMessage;
    type Properties = PaginationProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let total_pages = ctx.props().total_pages.unwrap_or_default();
        Self {
            total_pages,
            current_page: if total_pages > 0 { 1 } else { 0 },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PaginationMessage::Next => {
                if self.next_page() {
                    ctx.props().on_change.emit(self.current_page);
                    true
                } else {
                    false
                }
            }
            PaginationMessage::Prev => {
                if self.prev_page() {
                    ctx.props().on_change.emit(self.current_page);
                    true
                } else {
                    false
                }
            }
            PaginationMessage::First => {
                if self.first_page() {
                    ctx.props().on_change.emit(self.current_page);
                    true
                } else {
                    false
                }
            }
            PaginationMessage::Last => {
                if self.last_page() {
                    ctx.props().on_change.emit(self.current_page);
                    true
                } else {
                    false
                }
            }
            PaginationMessage::Set(page) => {
                if self.set_page(page) {
                    ctx.props().on_change.emit(self.current_page);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let edge_page_count = ctx.props().edge_page_count;
        let sibling_page_count = ctx.props().sibling_page_count;

        let front_breakpoint = edge_page_count + 2 + sibling_page_count;
        let rear_breakpoint = self
            .total_pages
            .saturating_sub(edge_page_count + 2 - 1 + sibling_page_count);

        let mid_page = if self.current_page < front_breakpoint {
            if front_breakpoint < self.total_pages {
                front_breakpoint
            } else {
                self.total_pages
            }
        } else if self.current_page > rear_breakpoint {
            rear_breakpoint
        } else {
            self.current_page
        };

        let is_front_truncated = mid_page > front_breakpoint;
        let is_rear_truncated = mid_page < rear_breakpoint;

        let front_items = html! {
            if is_front_truncated {
                { (1..=edge_page_count).map(|page| {
                        let is_active = page == self.current_page;
                        html! {
                            <button
                                class={is_active.then(|| "active")}
                                onclick={ctx.link().callback(move |_| Self::Message::Set(page))}
                            >
                                { page }
                            </button>
                        }
                    }).collect::<Html>() }
                <button
                    onclick={let current_page = self.current_page.clone();
                        ctx.link().callback(move |_| Self::Message::Set(current_page - 5))}
                >
                    { "..." }
                </button>
                { (mid_page-sibling_page_count..=mid_page).map(|page| {
                        let is_active = page == self.current_page;
                        html! {
                            <button
                                class={is_active.then(|| "active")}
                                onclick={ctx.link().callback(move |_| Self::Message::Set(page))}
                            >
                                { page }
                            </button>
                        }
                    }).collect::<Html>() }
            } else {
                { (1..=mid_page).map(|page| {
                        let is_active = page == self.current_page;
                        html! {
                            <button
                                class={is_active.then(|| "active")}
                                onclick={ctx.link().callback(move |_| Self::Message::Set(page))}
                            >
                                { page }
                            </button>
                        }
                    }).collect::<Html>() }
            }
        };

        let rear_items = html! {
            if is_rear_truncated {
                { (mid_page+1..=mid_page+sibling_page_count).map(|page| {
                        let is_active = page == self.current_page;
                        html! {
                            <button
                                class={is_active.then(|| "active")}
                                onclick={ctx.link().callback(move |_| Self::Message::Set(page))}
                            >
                                { page }
                            </button>
                        }
                    }).collect::<Html>() }
                <button
                    onclick={let current_page = self.current_page.clone();
                    ctx.link().callback(move |_| Self::Message::Set(current_page + 5))}
                >
                    { "..." }
                </button>
                { (self.total_pages-edge_page_count+1..=self.total_pages).map(|page| {
                        let is_active = page == self.current_page;
                        html! {
                            <button
                                class={is_active.then(|| "active")}
                                onclick={ctx.link().callback(move |_| Self::Message::Set(page))}
                            >
                                { page }
                            </button>
                        }
                    }).collect::<Html>() }
            } else {
                { (mid_page+1..=self.total_pages).map(|page| {
                        let is_active = page == self.current_page;
                        html! {
                            <button
                                class={is_active.then(|| "active")}
                                onclick={ctx.link().callback(move |_| Self::Message::Set(page))}
                            >
                                { page }
                            </button>
                        }
                    }).collect::<Html>() }
            }
        };

        html! {
            <div class="pagination-container">
                <button
                    onclick={ctx.link().callback(|_| Self::Message::First)}
                    disabled={self.current_page <= 1}
                >
                    { "First" }
                </button>
                <button
                    onclick={ctx.link().callback(|_| Self::Message::Prev)}
                    disabled={self.current_page <= 1}
                >
                    { "Prev" }
                </button>
                { front_items }
                { rear_items }
                <button
                    onclick={ctx.link().callback(|_| Self::Message::Next)}
                    disabled={self.current_page == self.total_pages}
                >
                    { "Next" }
                </button>
                <button
                    onclick={ctx.link().callback(|_| Self::Message::Last)}
                    disabled={self.current_page == self.total_pages}
                >
                    { "Last" }
                </button>
            </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination() {
        let total_pages = 5;
        let mut pagination = Pagination {
            total_pages,
            current_page: 1,
        };
        assert_eq!(pagination.current_page, 1);

        pagination.prev_page();
        assert_eq!(pagination.current_page, 1);

        pagination.next_page();
        assert_eq!(pagination.current_page, 2);

        pagination.prev_page();
        assert_eq!(pagination.current_page, 1);

        pagination.first_page();
        assert_eq!(pagination.current_page, 1);

        pagination.last_page();
        assert_eq!(pagination.current_page, total_pages);
    }
}

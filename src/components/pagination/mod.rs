use yew::prelude::*;

/// The Pagination component has the following props:
///
/// Required props:
///
/// - `total_pages`: The total number of pages.
///
/// Optional props:
///
/// - `edge_page_count`: To control number of the pages to show on the edge when
///   ellipsis button is shown, optional with default value `1`.
/// - `sibling_page_count`: To control number of the pages to show before and
///   after the current page, optional with default value `2`
///
/// Event handlers:
///
/// - `on_change`: Callback function, called when the page number changed.
#[derive(Debug, PartialEq, Properties)]
pub struct PaginationProperties {
    pub total_pages: Option<usize>,
    #[prop_or(1)]
    pub edge_page_count: usize,
    #[prop_or(1)]
    pub sibling_page_count: usize,
    #[prop_or_default]
    pub on_change: Callback<usize>,
}

#[derive(Debug)]
pub enum PaginationMessage {
    First,
    Last,
    Next,
    Prev,
    Set(usize),
}

/// The pagination component enables the user to navigate between multiple pages
///
/// Usage:
/// ```ignore
/// <Pagination
///     total_pages=24
///     // Optional
///     edge_page_count=1
///     // Optional
///     sibling_page_count=2
///     on_change={|page| log::info!("current page: {page}")}
/// />
/// ```
#[derive(Debug)]
pub struct Pagination {
    total_pages: usize,
    current_page: usize,
}

impl Pagination {
    pub fn set_page(&mut self, page: usize) -> bool {
        let page = page.clamp(1, self.total_pages);

        if page != self.current_page {
            self.current_page = page;
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
}

impl Component for Pagination {
    type Message = PaginationMessage;
    type Properties = PaginationProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let total_pages = ctx.props().total_pages.unwrap_or_default();

        Self {
            current_page: usize::from(total_pages > 0),
            total_pages,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
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
        let Self::Properties {
            edge_page_count,
            sibling_page_count,
            ..
        } = ctx.props();

        let front_breakpoint = edge_page_count + 2 + sibling_page_count;
        let rear_breakpoint = self
            .total_pages
            .saturating_sub(edge_page_count + 2 - 1 + sibling_page_count);

        let mid_page = match self.current_page {
            p if p < front_breakpoint => front_breakpoint.min(self.total_pages),
            p if p > rear_breakpoint => rear_breakpoint,
            _ => self.current_page,
        };

        let is_front_truncated = mid_page > front_breakpoint;
        let is_rear_truncated = mid_page < rear_breakpoint;

        let render_page_button = |page| {
            let is_active = page == self.current_page;
            html! {
                <button
                    class={classes!(is_active.then_some("active"))}
                    onclick={ctx.link().callback(move |_| Self::Message::Set(page))}
                >
                    { page }
                </button>
            }
        };

        let front_items = html! {
            if is_front_truncated {
                { (1..=*edge_page_count).map(render_page_button).collect::<Html>() }
                <button
                    onclick={let current_page = self.current_page;
                        ctx.link().callback(move |_| Self::Message::Set(current_page.saturating_sub(5)))}
                >
                    { "..." }
                </button>
                { (mid_page-sibling_page_count..=mid_page).map(render_page_button).collect::<Html>() }
            } else {
                { (1..=mid_page).map(render_page_button).collect::<Html>() }
            }
        };

        let rear_items = html! {
            if is_rear_truncated {
                { (mid_page+1..=mid_page+sibling_page_count).map(render_page_button).collect::<Html>() }
                <button
                    onclick={let current_page = self.current_page;
                    ctx.link().callback(move |_| Self::Message::Set(current_page + 5))}
                >
                    { "..." }
                </button>
                { (self.total_pages-edge_page_count+1..=self.total_pages).map(render_page_button).collect::<Html>() }
            } else {
                { (mid_page+1..=self.total_pages).map(render_page_button).collect::<Html>() }
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
    fn test_render_no_page() {
        let _ = html! { <Pagination total_pages=0 /> };
    }

    #[test]
    fn test_current_page_when_no_page() {
        let total_pages = 0;
        let pagination = Pagination {
            total_pages,
            current_page: usize::from(total_pages > 0),
        };
        assert_eq!(pagination.current_page, total_pages);
    }

    #[test]
    fn test_current_page_when_has_page() {
        let total_pages = 1;
        let pagination = Pagination {
            total_pages,
            current_page: usize::from(total_pages > 0),
        };
        assert_eq!(pagination.current_page, 1);

        let total_pages = 100;
        let pagination = Pagination {
            total_pages,
            current_page: usize::from(total_pages > 0),
        };
        assert_eq!(pagination.current_page, 1);
    }

    #[test]
    fn test_pagination() {
        let total_pages = 5;
        let mut pagination = Pagination {
            total_pages,
            current_page: 1,
        };
        assert_eq!(pagination.current_page, 1);

        _ = pagination.prev_page();
        assert_eq!(pagination.current_page, 1);

        _ = pagination.next_page();
        assert_eq!(pagination.current_page, 2);

        _ = pagination.prev_page();
        assert_eq!(pagination.current_page, 1);

        _ = pagination.first_page();
        assert_eq!(pagination.current_page, 1);

        _ = pagination.last_page();
        assert_eq!(pagination.current_page, total_pages);
    }
}

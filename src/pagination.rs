use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PaginationProperties {
    pub total_pages: Option<usize>,
    pub on_change: Callback<usize>,
}

#[derive(Debug)]
pub enum PaginationMessage {
    Next,
    Prev,
    First,
    Last,
    Set(usize),
}

#[derive(Debug, Clone)]
pub struct Pagination {
    total_pages: usize,
    current_page: usize,
}

impl Pagination {
    pub fn set_page(&mut self, page: usize) -> bool {
        if page > 0 && page <= self.total_pages && page != self.current_page {
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
                <>
                    { (1..=self.total_pages).map(|page| {
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
                </>
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

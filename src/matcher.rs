use druid::widget::prelude::*;
use druid::{Data, WidgetPod};
use std::mem::discriminant;

pub struct WidgetMatcher<D> {
    content: Option<WidgetPod<D, Box<dyn Widget<D>>>>,
    constructor: Box<dyn Fn(&D) -> Box<dyn Widget<D>>>,
}

impl<D> WidgetMatcher<D> {
    pub fn new<C>(constructor: C) -> Self
    where
        C: Fn(&D) -> Box<dyn Widget<D>> + 'static,
    {
        WidgetMatcher {
            content: None,
            constructor: Box::new(constructor),
        }
    }
}

impl<D: Data> Widget<D> for WidgetMatcher<D> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut D, env: &Env) {
        if let Some(content) = &mut self.content {
            content.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &D, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.content = Some(WidgetPod::new((self.constructor)(data)));
            if self.content.is_some() {
                ctx.children_changed();
            }
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &D, data: &D, env: &Env) {
        if discriminant(old_data) != discriminant(data) {
            self.content = Some(WidgetPod::new((self.constructor)(data)));
            if self.content.is_some() {
                ctx.children_changed();
            }
        } else {
            if let Some(content) = &mut self.content {
                content.update(ctx, data, env);
            }
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &D, env: &Env) -> Size {
        self.content
            .as_mut()
            .map(|c| c.layout(ctx, bc, data, env))
            .unwrap_or_default()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &D, env: &Env) {
        if let Some(content) = &mut self.content {
            content.paint(ctx, data, env);
        }
    }
}

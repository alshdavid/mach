pub mod resolver_javascript;
pub mod resolver_rpc;
pub mod transformer_css;
pub mod transformer_drop;
pub mod transformer_html;
pub mod transformer_javascript;
pub mod transformer_json;
pub mod transformer_rpc;

pub enum Resolver {
  JavaScript(resolver_javascript::ResolverJavaScript),
  RPC(resolver_rpc::ResolverAdapter),
}

pub enum Transformer {
  CSS(transformer_css::TransformerCSS),
  HTML(transformer_html::TransformerHtml),
  JavaScript(transformer_javascript::TransformerJavaScript),
  Drop(transformer_drop::TransformerDrop),
  JSON(transformer_json::TransformerJson),
  RPC(transformer_rpc::TransformerAdapter),
}
import "@typespec/http";
import "@typespec/openapi3";
import "@typespec/openapi";

import "../types.tsp";

using Http;
@service(#{ title: "Widget Service" })
namespace DemoService;

@route("/widgets")
@tag("Widgets")
interface Widgets {
  /** List widgets */
  @get object(): Object;
}

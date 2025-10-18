import "@typespec/http";
import "@typespec/openapi3";
import "@typespec/openapi";

import "../types.tsp";

using Http;
@service(#{ title: "Test Service" })
namespace DemoService;

@route("/date")
@tag("date")
interface Date {
  /** List widgets */
  @get date(): utcDateTime;
}

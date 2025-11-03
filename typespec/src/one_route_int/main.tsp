import "@typespec/http";
import "@typespec/openapi3";
import "@typespec/openapi";

import "../types.tsp";

using Http;
using OpenAPI;

@service(#{ title: "Widget Service" })
namespace DemoService;

@route("/widgets")
@tag("Widgets")
interface Widgets {
  /** Get integer */
  @operationId("GetAllIntegers")
  @get
  object(): int32;
}

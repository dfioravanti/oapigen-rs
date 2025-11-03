import "@typespec/http";
import "@typespec/openapi3";
import "@typespec/openapi";

import "../types.tsp";

using OpenAPI;
using Http;

@service(#{ title: "Test Service" })
namespace DemoService;

@route("/nullable")
interface Nullable {
  /** Get the current date */
  @operationId("GetNullable")
  @get
  nullable(): {
    @header contentType: "application/json";
    @bodyRoot _: string | null;
  };
}

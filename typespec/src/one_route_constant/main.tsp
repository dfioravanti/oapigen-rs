import "@typespec/http";
import "@typespec/openapi3";
import "@typespec/openapi";

import "../types.tsp";

using OpenAPI;
using Http;

@service(#{ title: "Test Service" })
namespace DemoService;

@route("/three")
@tag("number")
interface Number {
  /** Get the current date */
  @operationId("GetThree")
  @get
  three(): 3;
}

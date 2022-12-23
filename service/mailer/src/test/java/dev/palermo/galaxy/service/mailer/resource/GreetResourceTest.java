package dev.palermo.galaxy.service.mailer.resource;

import static io.restassured.RestAssured.when;
import static org.hamcrest.core.IsEqual.equalTo;

import dev.palermo.galaxy.service.mailer.BaseRestTest;
import org.junit.Test;

public class GreetResourceTest extends BaseRestTest {

  @Test
  public void greetTest() {
    when()
        .request("GET", "/greet")
        .then()
        .statusCode(200)
        .body(equalTo("Hello World!"));
  }

}

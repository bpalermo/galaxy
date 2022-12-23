package dev.palermo.galaxy.service.mailer.actuator;

import static io.restassured.RestAssured.when;

import dev.palermo.galaxy.service.mailer.BaseRestTest;
import org.junit.Test;

public class ActuatorTest extends BaseRestTest {

  @Test
  public void healthTest() {
    when()
        .request("GET", "/-/-/health/")
        .then()
        .statusCode(200);
  }

}

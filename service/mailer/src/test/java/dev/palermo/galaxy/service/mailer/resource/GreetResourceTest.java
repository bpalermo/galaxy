package dev.palermo.galaxy.service.mailer.resource;

import static io.restassured.RestAssured.when;
import static org.hamcrest.core.IsEqual.equalTo;

import dev.palermo.galaxy.service.mailer.App;
import io.restassured.RestAssured;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.boot.test.web.server.LocalServerPort;
import org.springframework.test.context.junit4.SpringRunner;

@RunWith(SpringRunner.class)
@SpringBootTest(
    webEnvironment = SpringBootTest.WebEnvironment.RANDOM_PORT,
    classes = {
        App.class
    }
)
public class GreetResourceTest {

  @LocalServerPort
  int port;

  @Before
  public void setUp() {
    RestAssured.port = port;
  }

  @Test
  public void greetTest() {
    when()
        .request("GET", "/greet")
        .then()
        .statusCode(200)
        .body(equalTo("Hello World!"));
  }

}

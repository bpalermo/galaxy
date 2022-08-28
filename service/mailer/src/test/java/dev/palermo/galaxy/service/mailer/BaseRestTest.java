package dev.palermo.galaxy.service.mailer;

import io.restassured.RestAssured;
import org.junit.Before;
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
public abstract class BaseRestTest {
  @LocalServerPort
  int port;

  @Before
  public void setUp() {
    RestAssured.port = port;
  }
}

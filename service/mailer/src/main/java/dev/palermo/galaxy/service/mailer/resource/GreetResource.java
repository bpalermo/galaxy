package dev.palermo.galaxy.service.mailer.resource;

import org.springframework.beans.factory.annotation.Value;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class GreetResource {

  @Value("${app.message}")
  String message;

  @GetMapping("/greet")
  public String greet(@RequestParam(value = "name", defaultValue = "World") String name) {
    return String.format("%s %s!", message, name);
  }
}

package jlox;

import java.util.HashMap;
import java.util.Map;

class Environment {
  private final Map<String, Object> values = new HashMap<>();
  final Environment enclosing;

  Environment() {
    enclosing = null;
  }

  Environment(Environment enclosing) {
    this.enclosing = enclosing;
  }

  Object get(Token name) {
    // System.out.println("get " + name.lexeme + "from " + s());
    if (values.containsKey(name.lexeme)) {
      Object val = values.get(name.lexeme);
      // System.out.println("got " + val);
      return val;
    }

    if (enclosing != null) return enclosing.get(name);

    throw new RuntimeError(name,
        "Undefined variable '" + name.lexeme + "'.");
  }

  void define(String name, Object value) {
    values.put(name, value);
  }

  void assign(Token name, Object value) {
    if(values.containsKey(name.lexeme)) {
      values.put(name.lexeme, value);
      return;
    }

    if (enclosing != null) {
      enclosing.assign(name, value);
      return;
    }

    throw new RuntimeError(name,
        "Undefined variable '"+ name.lexeme + "'.");
  }

  public String s() {
    return values.toString();
  }
}

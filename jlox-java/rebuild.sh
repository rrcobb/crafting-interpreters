javac jlox-java/tool/GenerateAst.java && \
java -cp jlox-java/ tool.GenerateAst jlox-java/jlox/ && \
javac jlox-java/jlox/*.java && \
java -cp jlox-java/ jlox.Lox $1

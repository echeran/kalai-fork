package b;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.ArrayList;
public class Variable {
public static final long sideEffect() {
long y = 2;
{
y = 3;
return y;
}
}
public static final void main(String[] args) {
System.out.println(b.Variable.sideEffect());
}
}
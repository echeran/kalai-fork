package datadiff;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.ArrayList;
import java.util.stream.Collectors;
import kalai.Kalai;
import kalai.Kalai.*;
public class Examples {
public static final void main(String[] _args) {
final Object a = (Object)new io.lacuna.bifurcan.List<Object>().addLast(1L).addLast(2L);
final Object b = (Object)new io.lacuna.bifurcan.List<Object>().addLast(1L).addLast(3L);
final Object v = datadiff.Core.diff(a, b);
System.out.println(v);
}
}

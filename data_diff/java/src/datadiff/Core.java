package datadiff;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.ArrayList;
import java.util.stream.Collectors;
import kalai.Kalai;
public class Core {
public static final TYPE_MISSING diffAssociativeKey(final TYPE_MISSING a, final TYPE_MISSING b, final TYPE_MISSING k) {
final Object va = a.get(k);
final Object vb = b.get(k);
final Object vec18742 = diff(va, vb);
Object tmp43 = null;
if ((0L <= 0L))
{
if ((0L < vec__18742.length()))
{
tmp43 = vec__18742.get();
}
}
final Object aa = tmp43;
Object tmp44 = null;
if ((0L <= 1L))
{
if ((1L < vec__18742.length()))
{
tmp44 = vec__18742.get();
}
}
final Object bb = tmp44;
Object tmp45 = null;
if ((0L <= 2L))
{
if ((2L < vec__18742.length()))
{
tmp45 = vec__18742.get();
}
}
final Object ab = tmp45;
final boolean inA = a.containsKey(k);
final boolean inB = b.containsKey(k);
final boolean d = !(ab == null);
final boolean or5581Auto = d;
boolean tmp1;
boolean tmp2 = or__5581__auto__;
if (tmp2)
{
tmp1 = or__5581__auto__;
}
else
{
final boolean and5579Auto = (va == null);
"MISSING_TYPE" tmp3;
boolean tmp4 = and__5579__auto__;
if (tmp4)
{
tmp3 = (vb == null);
}
else
{
tmp3 = and__5579__auto__;
}
{
tmp1 = tmp3;
}
}
final boolean c = tmp1;
final boolean and5579Auto = inA;
boolean tmp5;
boolean tmp6 = and__5579__auto__;
if (tmp6)
{
final boolean and5579Auto = inB;
boolean tmp7;
boolean tmp8 = and__5579__auto__;
if (tmp8)
{
tmp7 = c;
}
else
{
tmp7 = and__5579__auto__;
}
{
tmp5 = tmp7;
}
}
else
{
tmp5 = and__5579__auto__;
}
final boolean same = tmp5;
final boolean e = !(aa == null);
final boolean f = !(bb == null);
final boolean or5581Auto = e;
boolean tmp9;
boolean tmp10 = or__5581__auto__;
if (tmp10)
{
tmp9 = or__5581__auto__;
}
else
{
tmp9 = !same;
}
final boolean g = tmp9;
final boolean or5581Auto = f;
boolean tmp11;
boolean tmp12 = or__5581__auto__;
if (tmp12)
{
tmp11 = or__5581__auto__;
}
else
{
tmp11 = !same;
}
final boolean h = tmp11;
io.lacuna.bifurcan.Map<Object,Object> tmp13;
final boolean and5579Auto = inA;
boolean tmp15;
boolean tmp16 = and__5579__auto__;
if (tmp16)
{
tmp15 = g;
}
else
{
tmp15 = and__5579__auto__;
}
boolean tmp14 = tmp15;
if (tmp14)
{
tmp13 = (Object)new io.lacuna.bifurcan.Map<Object,Object>().put(k, aa, io.lacuna.bifurcan.Maps.MERGE_LAST_WRITE_WINS);
}
io.lacuna.bifurcan.Map<Object,Object> tmp17;
final boolean and5579Auto = inB;
boolean tmp19;
boolean tmp20 = and__5579__auto__;
if (tmp20)
{
tmp19 = h;
}
else
{
tmp19 = and__5579__auto__;
}
boolean tmp18 = tmp19;
if (tmp18)
{
tmp17 = (Object)new io.lacuna.bifurcan.Map<Object,Object>().put(k, bb, io.lacuna.bifurcan.Maps.MERGE_LAST_WRITE_WINS);
}
io.lacuna.bifurcan.Map<Object,Object> tmp21;
boolean tmp22 = same;
if (tmp22)
{
tmp21 = (Object)new io.lacuna.bifurcan.Map<Object,Object>().put(k, ab, io.lacuna.bifurcan.Maps.MERGE_LAST_WRITE_WINS);
}
return (Object)new io.lacuna.bifurcan.List<Object>().addLast(tmp13).addLast(tmp17).addLast(tmp21);
}
public static final TYPE_MISSING merge2(final TYPE_MISSING m1, final TYPE_MISSING m2) {
return kalai.Kalai.foldLeft(m2.stream(), m1, conj);
}
public static final TYPE_MISSING mergeDiffs(final Object diff1, final Object diff2) {
return (Object)vec(kalai.Kalai.map((a, b) -> {
return merge2(a, b);
}, diff1.stream(), diff2.stream()));
}
public static final TYPE_MISSING diffAssociative(final TYPE_MISSING a, final TYPE_MISSING b, final TYPE_MISSING ks) {
return kalai.Kalai.foldLeft(ks.stream().map((k) -> {
return diffAssociativeKey(a, b, k);
}), (Object)new io.lacuna.bifurcan.List<Object>().addLast(null).addLast(null).addLast(null), mergeDiffs);
}
public static final TYPE_MISSING union(final TYPE_MISSING s1, final TYPE_MISSING s2) {
boolean tmp23 = (s1.length() < s2.length());
if (tmp23)
{
return kalai.Kalai.foldLeft(s1.stream(), s2, conj);
}
else
{
return kalai.Kalai.foldLeft(s2.stream(), s1, conj);
}
}
public static final TYPE_MISSING difference(final TYPE_MISSING s1, final TYPE_MISSING s2) {
boolean tmp24 = (s1.length() < s2.length());
if (tmp24)
{
return kalai.Kalai.foldLeft(s1.stream(), s1, (result, item) -> {
boolean tmp25 = s2.containsKey(item);
if (tmp25)
{
return disj(result, item);
}
else
{
return result;
}
});
}
else
{
return kalai.Kalai.foldLeft(s2.stream(), s1, disj);
}
}
public static final TYPE_MISSING intersection(final TYPE_MISSING s1, final TYPE_MISSING s2) {
boolean tmp26 = (s2.length() < s1.length());
if (tmp26)
{
return intersection(s2, s1);
}
else
{
return kalai.Kalai.foldLeft(s1.stream(), s1, (result, item) -> {
boolean tmp27 = s2.containsKey(item);
if (tmp27)
{
return result;
}
else
{
return disj(result, item);
}
});
}
}
public static final TYPE_MISSING atomDiff(final TYPE_MISSING a, final TYPE_MISSING b) {
boolean tmp28 = (a == b);
if (tmp28)
{
return (Object)new io.lacuna.bifurcan.List<Object>().addLast(null).addLast(null).addLast(a);
}
else
{
return (Object)new io.lacuna.bifurcan.List<Object>().addLast(a).addLast(b).addLast(null);
}
}
public static final String equalityPartition(final TYPE_MISSING x) {
boolean tmp29 = (x instanceof Set);
if (tmp29)
{
return ":set";
}
else
{
boolean tmp30 = (x instanceof Map);
if (tmp30)
{
return ":map";
}
else
{
boolean tmp31 = (x instanceof List);
if (tmp31)
{
return ":sequence";
}
else
{
return ":atom";
}
}
}
}
public static final TYPE_MISSING mapDiff(final TYPE_MISSING a, final TYPE_MISSING b) {
final Object abKeys = union(keys(a), keys(b));
return diffAssociative(a, b, abKeys);
}
public static final TYPE_MISSING setDiff(final TYPE_MISSING a, final TYPE_MISSING b) {
return (Object)new io.lacuna.bifurcan.List<Object>().addLast(difference(a, b)).addLast(difference(b, a)).addLast(intersection(a, b));
}
public static final TYPE_MISSING vectorize(final TYPE_MISSING m) {
boolean tmp32 = notEmpty(m);
if (tmp32)
{
Object tmp46 = null;
if ((0L <= 0L))
{
if ((0L < vec__18786.length()))
{
tmp46 = vec__18786.get();
}
}
Object tmp47 = null;
if ((0L <= 1L))
{
if ((1L < vec__18786.length()))
{
tmp47 = vec__18786.get();
}
}
{
return kalai.Kalai.foldLeft(m.stream(), (Object)vec(repeat((long)kalai.Kalai.foldLeft(keys(m).stream(), keys(m).stream().findFirst().get(), (a, b) -> {
final long aInt = (long)a;
final long bInt = (long)b;
return (Object)clojure.lang.Numbers.max(aInt, bInt);
}), null)), (result, p__18784) -> {
final TYPE_MISSING vec18786 = p__18784;
final Object k = tmp46;
final Object v = tmp47;
return result.put(k, v);
});
}
}
else
{
return null;
}
}
public static final TYPE_MISSING sequenceDiff(final TYPE_MISSING a, final TYPE_MISSING b) {
"MISSING_TYPE" tmp33;
boolean tmp34 = (a instanceof List);
if (tmp34)
{
tmp33 = a;
}
else
{
tmp33 = (Object)vec(a.stream());
}
"MISSING_TYPE" tmp35;
boolean tmp36 = (b instanceof List);
if (tmp36)
{
tmp35 = b;
}
else
{
tmp35 = (Object)vec(b.stream());
}
return (Object)vec(diffAssociative(tmp33, tmp35, (Object)vec(range(clojure.lang.Numbers.max(a.length(), b.length())))).stream().map((a) -> {
return vectorize(a);
}));
}
public static final TYPE_MISSING diffSimilar(final TYPE_MISSING a, final TYPE_MISSING b) {
final String partitionA = equalityPartition(a);
final String partitionB = equalityPartition(b);
{
boolean tmp37 = partitionA.equals(partitionB);
if (tmp37)
{
boolean tmp38 = (partitionA == ":set");
if (tmp38)
{
return setDiff(a, b);
}
else
{
boolean tmp39 = (partitionA == ":map");
if (tmp39)
{
return mapDiff(a, b);
}
else
{
boolean tmp40 = (partitionA == ":sequence");
if (tmp40)
{
return sequenceDiff(a, b);
}
else
{
boolean tmp41 = (partitionA == ":atom");
if (tmp41)
{
return atomDiff(a, b);
}
else
{
return null;
}
}
}
}
}
else
{
return atomDiff(a, b);
}
}
}
public static final TYPE_MISSING diff(final Object a, final Object b) {
boolean tmp42 = (a == b);
if (tmp42)
{
return (Object)new io.lacuna.bifurcan.List<Object>().addLast(null).addLast(null).addLast(a);
}
else
{
return diffSimilar(a, b);
}
}
}

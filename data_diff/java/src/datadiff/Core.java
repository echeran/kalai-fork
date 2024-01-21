package datadiff;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.ArrayList;
import java.util.stream.Collectors;
import kalai.Kalai;
import kalai.Kalai.*;
public class Core {
public static final Object diffAssociativeKey(final Object a, final Object b, final Object k) {
final Object va = kalai.Kalai.get(a, k);
final Object vb = kalai.Kalai.get(b, k);
final Object vec__18696 = diff(va, vb);
final Object aa = kalai.Kalai.nth(vec__18696, 0L, null);
final Object bb = kalai.Kalai.nth(vec__18696, 1L, null);
final Object ab = kalai.Kalai.nth(vec__18696, 2L, null);
final boolean inA = kalai.Kalai.contains(a, k);
final boolean inB = kalai.Kalai.contains(b, k);
final boolean d = !(ab == null);
final boolean or__5581__auto__ = d;
boolean tmp1;
boolean tmp2 = or__5581__auto__;
if (tmp2)
{
tmp1 = or__5581__auto__;
}
else
{
final boolean and__5579__auto__ = (va == null);
boolean tmp3;
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
final boolean and__5579__auto___1 = inA;
boolean tmp5;
boolean tmp6 = and__5579__auto___1;
if (tmp6)
{
final boolean and__5579__auto___2 = inB;
boolean tmp7;
boolean tmp8 = and__5579__auto___2;
if (tmp8)
{
tmp7 = c;
}
else
{
tmp7 = and__5579__auto___2;
}
{
tmp5 = tmp7;
}
}
else
{
tmp5 = and__5579__auto___1;
}
final boolean same = tmp5;
final boolean e = !(aa == null);
final boolean f = !(bb == null);
final boolean or__5581__auto___1 = e;
boolean tmp9;
boolean tmp10 = or__5581__auto___1;
if (tmp10)
{
tmp9 = or__5581__auto___1;
}
else
{
tmp9 = !same;
}
final boolean g = tmp9;
final boolean or__5581__auto___2 = f;
boolean tmp11;
boolean tmp12 = or__5581__auto___2;
if (tmp12)
{
tmp11 = or__5581__auto___2;
}
else
{
tmp11 = !same;
}
final boolean h = tmp11;
io.lacuna.bifurcan.Map<Object,Object> tmp13;
final boolean and__5579__auto___3 = inA;
boolean tmp15;
boolean tmp16 = and__5579__auto___3;
if (tmp16)
{
tmp15 = g;
}
else
{
tmp15 = and__5579__auto___3;
}
boolean tmp14 = tmp15;
if (tmp14)
{
tmp13 = (Object)new io.lacuna.bifurcan.Map<Object,Object>().put(k, aa, io.lacuna.bifurcan.Maps.MERGE_LAST_WRITE_WINS);
}
io.lacuna.bifurcan.Map<Object,Object> tmp17;
final boolean and__5579__auto___4 = inB;
boolean tmp19;
boolean tmp20 = and__5579__auto___4;
if (tmp20)
{
tmp19 = h;
}
else
{
tmp19 = and__5579__auto___4;
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
public static final Object merge2(final Object m1, final Object m2) {
return kalai.Kalai.foldLeft(m2.stream(), m1, conj);
}
public static final Object mergeDiffs(final Object diff1, final Object diff2) {
return (Object)kalai.Kalai.vec(kalai.Kalai.map((a, b) -> {
return merge2(a, b);
}, diff1.stream(), diff2.stream()));
}
public static final Object diffAssociative(final Object a, final Object b, final Object ks) {
return kalai.Kalai.foldLeft(ks.stream().map((k) -> {
return diffAssociativeKey(a, b, k);
}), (Object)new io.lacuna.bifurcan.List<Object>().addLast(null).addLast(null).addLast(null), mergeDiffs);
}
public static final Object union(final Object s1, final Object s2) {
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
public static final Object difference(final Object s1, final Object s2) {
boolean tmp24 = (s1.length() < s2.length());
if (tmp24)
{
return kalai.Kalai.foldLeft(s1.stream(), s1, (result, item) -> {
boolean tmp25 = kalai.Kalai.contains(s2, item);
if (tmp25)
{
return kalai.Kalai.disj(result, item);
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
public static final Object intersection(final Object s1, final Object s2) {
boolean tmp26 = (s2.length() < s1.length());
if (tmp26)
{
return intersection(s2, s1);
}
else
{
return kalai.Kalai.foldLeft(s1.stream(), s1, (result, item) -> {
boolean tmp27 = kalai.Kalai.contains(s2, item);
if (tmp27)
{
return result;
}
else
{
return kalai.Kalai.disj(result, item);
}
});
}
}
public static final Object atomDiff(final Object a, final Object b) {
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
public static final String equalityPartition(final Object x) {
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
public static final Object mapDiff(final Object a, final Object b) {
final Object abKeys = union(kalai.Kalai.keys(a), kalai.Kalai.keys(b));
return diffAssociative(a, b, abKeys);
}
public static final Object setDiff(final Object a, final Object b) {
return (Object)new io.lacuna.bifurcan.List<Object>().addLast(difference(a, b)).addLast(difference(b, a)).addLast(intersection(a, b));
}
public static final Object vectorize(final Object m) {
boolean tmp32 = kalai.Kalai.notEmpty(m);
if (tmp32)
{
return kalai.Kalai.foldLeft(m.stream(), (Object)kalai.Kalai.vec(kalai.Kalai.repeat((long)kalai.Kalai.foldLeft(kalai.Kalai.keys(m).stream(), kalai.Kalai.keys(m).stream().findFirst().get(), (a, b) -> {
final long aInt = (long)a;
final long bInt = (long)b;
return (Object)clojure.lang.Numbers/max(aInt, bInt);
}), null)), (result, p__18738) -> {
final Object vec__18740 = p__18738;
final Object k = kalai.Kalai.nth(vec__18740, 0L, null);
final Object v = kalai.Kalai.nth(vec__18740, 1L, null);
return result.put(k, v);
});
}
else
{
return null;
}
}
public static final Object sequenceDiff(final Object a, final Object b) {
Object tmp33;
boolean tmp34 = (a instanceof List);
if (tmp34)
{
tmp33 = a;
}
else
{
tmp33 = (Object)kalai.Kalai.vec(a.stream());
}
Object tmp35;
boolean tmp36 = (b instanceof List);
if (tmp36)
{
tmp35 = b;
}
else
{
tmp35 = (Object)kalai.Kalai.vec(b.stream());
}
return (Object)kalai.Kalai.vec(diffAssociative(tmp33, tmp35, (Object)kalai.Kalai.vec(kalai.Kalai.range(clojure.lang.Numbers/max(a.length(), b.length())))).stream().map((a) -> {
return vectorize(a);
}));
}
public static final Object diffSimilar(final Object a, final Object b) {
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
public static final Object diff(final Object a, final Object b) {
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

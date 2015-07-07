public class ChopRecursive<T> implements Chop<T> {
    private int recursiveChop(T item, Comparable<T>[] array, int low, int width) {
        int index = low + width / 2;
        if(width <= 0) {
            return -1;
        } else if(array[index].compareTo(item) > 0) {
            return recursiveChop(item, array, low, width / 2);
        } else if(array[index].compareTo(item) < 0) {
            return recursiveChop(item, array, index + 1, (width - 1) / 2);
        } else {
            return index;
        }
    }

    public int chop(T item, Comparable<T>[] array) {
        return recursiveChop(item, array, 0, array.length);
    }
}

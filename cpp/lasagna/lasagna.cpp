// ovenTime returns the amount in minutes that the lasagna should stay in the
// oven.
int ovenTime() {
    return 40;
}

int remainingOvenTime(int actualMinutesInOven) {
    if (actualMinutesInOven > ovenTime()) {
        return 0;
    }
    return ovenTime() - actualMinutesInOven;
}

/* preparationTime returns an estimate of the preparation time based on the
   number of layers and the necessary time per layer.
*/
constexpr int preparationTime(int numberOfLayers) {
    return numberOfLayers * 2;
}

// elapsedTime calculates the total time spent to create and bake the lasagna so
// far.
int elapsedTime(int numberOfLayers, int actualMinutesInOven) {
    return preparationTime(numberOfLayers) + actualMinutesInOven;
}

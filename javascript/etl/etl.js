//
// This is only a SKELETON file for the 'ETL' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export const transform = (oldData) => {
    // oldData format:  { 1: ['A', 'E'], 2: ['D', 'G'] }
    // newData format = { a: 1, e: 1, d: 2, g: 2, };
    let newData = {}
    Object.entries(oldData).forEach((point, scribble) =>
        newData[scribble] = String(point).toLowerCase()
    )
    return newData
};

export default interface Teacher {
    id: number;
    name: string,
    grades: string, // "6,3,5..."
    total_lessions: number,
    present: number,
    absent: number,
    late: number,
}

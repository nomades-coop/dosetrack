export const getDateFrom = (value) => {
  try {
    value = new Date(parseInt(value.$date.$numberLong));
  } catch {
    value = new Date(value); //.toISOString().split("T")[0];
  }
  return `${value.getUTCDate()}-${value.getUTCMonth() + 1}-${value.getUTCFullYear()}`;
};
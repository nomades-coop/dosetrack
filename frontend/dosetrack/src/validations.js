export const isRequiredRule = (value) => {
  return value != null && value !== "";
};

export const isCommaSeparated = (value) => {
  return /.*,{1}.*/.test(value)
}

export const isDateRule = (value) => {
  let d = new Date(value);
  return d instanceof Date && !isNaN(d);
};

export const isFormValid = (errors) => {
  return !Object.keys(errors).some((inputName) =>
    Object.keys(errors[inputName]).some(
      (errorName) => errors[inputName][errorName]
    )
  );
};

export const setError = (errors, data, fieldName, callBack, msgError) => {
  if (!callBack(data[fieldName])) {
    errors[fieldName] = errors[fieldName] || { error: [] };
    errors[fieldName] = {
      ...errors[fieldName],
    };
    errors[fieldName]["error"].push(msgError);
  }
};
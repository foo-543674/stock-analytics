import { z } from 'zod';
import { parseApiResponse } from './parseApiResponse';

export const validationErrorSchema = z.object({
  fields: z.array(
    z.object({
      name: z.string(),
      constraints: z.array(
        z.object({
          rule: z.enum([
            'validation.required',
            'validation.duplicate',
            'validation.length_equals',
            'validation.numeric_only',
            'validation.ulid',
            'validation.resource_not_found',
            'validation.max_length',
          ]),
          args: z.array(z.string()).optional(),
        }),
      ),
    }),
  ),
});
export type ValidationError = z.infer<typeof validationErrorSchema>;
export type ValidationConstraint =
  ValidationError['fields'][number]['constraints'][number];
export type ValidationRule = ValidationConstraint['rule'];

export const parseValidationError = parseApiResponse(validationErrorSchema);

export const getConstraints = (fieldName: string, error: ValidationError) => {
  const field = error.fields.find(f => f.name === fieldName);
  return field?.constraints ?? [];
};

export const hasError = (fieldName: string, error: ValidationError) => {
  return error.fields.some(f => f.name === fieldName) ?? false;
};

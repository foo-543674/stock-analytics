import { Result } from '@/utils/Result';
import { ParseError } from './ParseError';

export type ParseResult<T> = Result<T, ParseError>;

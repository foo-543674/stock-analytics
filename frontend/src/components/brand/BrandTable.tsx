import {
  CircularProgress,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableRow,
} from '@mui/material';
import { Brand } from '@/components/brand/Schema';

export type BrandTableProps = {
  items: Brand[];
  loading: boolean;
};

export const BrandTable = (props: BrandTableProps) => {
  return (
    <>
      <Table>
        {/* Table Header */}
        <TableHead>
          <TableRow>
            <TableCell>Brand Name</TableCell>
            <TableCell>Brand Description</TableCell>
            <TableCell>Brand Image</TableCell>
            <TableCell>Actions</TableCell>
          </TableRow>
        </TableHead>
        {/* Table Body */}
        <TableBody>
          {props.loading ? (
            <TableRow>
              <TableCell colSpan={4} align="center">
                <CircularProgress />
              </TableCell>
            </TableRow>
          ) : (
            <TableRow>
              <TableCell>Brand Name</TableCell>
              <TableCell>Brand Description</TableCell>
              <TableCell>Brand Image</TableCell>
              <TableCell>Actions</TableCell>
            </TableRow>
          )}
        </TableBody>
      </Table>
    </>
  );
};

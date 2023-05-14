// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use super::*;

pub trait StackMatches<N: Network> {
    /// Checks that the given value matches the layout of the value type.
    fn matches_value_type(&self, value: &Value<N>, value_type: &ValueType<N>) -> Result<()>;

    /// Checks that the given stack value matches the layout of the register type.
    fn matches_register_type(&self, stack_value: &Value<N>, register_type: &RegisterType<N>) -> Result<()>;

    /// Checks that the given record matches the layout of the external record type.
    fn matches_external_record(&self, record: &Record<N, Plaintext<N>>, locator: &Locator<N>) -> Result<()>;

    /// Checks that the given record matches the layout of the record type.
    fn matches_record(&self, record: &Record<N, Plaintext<N>>, record_name: &Identifier<N>) -> Result<()>;

    /// Checks that the given plaintext matches the layout of the plaintext type.
    fn matches_plaintext(&self, plaintext: &Plaintext<N>, plaintext_type: &PlaintextType<N>) -> Result<()>;
}

pub trait StackProgram<N: Network> {
    /// Returns the program.
    fn program(&self) -> &Program<N>;

    /// Returns the program ID.
    fn program_id(&self) -> &ProgramID<N>;

    /// Returns `true` if the stack contains the external record.
    fn contains_external_record(&self, locator: &Locator<N>) -> bool;

    /// Returns the external stack for the given program ID.
    fn get_external_stack(&self, program_id: &ProgramID<N>) -> Result<&Self>;

    /// Returns the external program for the given program ID.
    fn get_external_program(&self, program_id: &ProgramID<N>) -> Result<&Program<N>>;

    /// Returns `true` if the stack contains the external record.
    fn get_external_record(&self, locator: &Locator<N>) -> Result<RecordType<N>>;

    /// Returns the function with the given function name.
    fn get_function(&self, function_name: &Identifier<N>) -> Result<Function<N>>;

    /// Returns the expected number of calls for the given function name.
    fn get_number_of_calls(&self, function_name: &Identifier<N>) -> Result<usize>;

    /// Returns the register types for the given closure or function name.
    fn get_register_types(&self, name: &Identifier<N>) -> Result<&RegisterTypes<N>>;

    /// Returns the register types for the given finalize name.
    fn get_finalize_types(&self, name: &Identifier<N>) -> Result<&FinalizeTypes<N>>;
}

pub trait RegistersCall<N: Network> {
    /// Returns the current call stack.
    fn call_stack(&self) -> CallStack<N>;
}

pub trait RegistersCaller<N: Network> {
    /// Returns the transition caller.
    fn caller(&self) -> Result<Address<N>>;

    /// Sets the transition caller.
    fn set_caller(&mut self, caller: Address<N>);

    /// Returns the transition view key.
    fn tvk(&self) -> Result<Field<N>>;

    /// Sets the transition view key.
    fn set_tvk(&mut self, tvk: Field<N>);
}

pub trait RegistersCallerCircuit<N: Network, A: circuit::Aleo<Network = N>> {
    /// Returns the transition caller, as a circuit.
    fn caller_circuit(&self) -> Result<circuit::Address<A>>;

    /// Sets the transition caller, as a circuit.
    fn set_caller_circuit(&mut self, caller_circuit: circuit::Address<A>);

    /// Returns the transition view key, as a circuit.
    fn tvk_circuit(&self) -> Result<circuit::Field<A>>;

    /// Sets the transition view key, as a circuit.
    fn set_tvk_circuit(&mut self, tvk_circuit: circuit::Field<A>);
}

pub trait RegistersLoad<N: Network> {
    /// Loads the value of a given operand.
    ///
    /// # Errors
    /// This method should halt if the register locator is not found.
    /// In the case of register members, this method should halt if the member is not found.
    fn load(&self, stack: &(impl StackMatches<N> + StackProgram<N>), operand: &Operand<N>) -> Result<Value<N>>;

    /// Loads the literal of a given operand.
    ///
    /// # Errors
    /// This method should halt if the given operand is not a literal.
    /// This method should halt if the register locator is not found.
    /// In the case of register members, this method should halt if the member is not found.
    #[inline]
    fn load_literal(
        &self,
        stack: &(impl StackMatches<N> + StackProgram<N>),
        operand: &Operand<N>,
    ) -> Result<Literal<N>> {
        match self.load(stack, operand)? {
            Value::Plaintext(Plaintext::Literal(literal, ..)) => Ok(literal),
            Value::Plaintext(Plaintext::Struct(..)) => bail!("Operand must be a literal"),
            Value::Record(..) => bail!("Operand must be a literal"),
        }
    }

    /// Loads the plaintext of a given operand.
    ///
    /// # Errors
    /// This method should halt if the given operand is not a plaintext.
    /// This method should halt if the register locator is not found.
    /// In the case of register members, this method should halt if the member is not found.
    #[inline]
    fn load_plaintext(
        &self,
        stack: &(impl StackMatches<N> + StackProgram<N>),
        operand: &Operand<N>,
    ) -> Result<Plaintext<N>> {
        match self.load(stack, operand)? {
            Value::Plaintext(plaintext) => Ok(plaintext),
            Value::Record(..) => bail!("Operand must be a plaintext"),
        }
    }
}

pub trait RegistersLoadCircuit<N: Network, A: circuit::Aleo<Network = N>> {
    /// Loads the value of a given operand.
    ///
    /// # Errors
    /// This method should halt if the register locator is not found.
    /// In the case of register members, this method should halt if the member is not found.
    fn load_circuit(
        &self,
        stack: &(impl StackMatches<N> + StackProgram<N>),
        operand: &Operand<N>,
    ) -> Result<circuit::Value<A>>;

    /// Loads the literal of a given operand.
    ///
    /// # Errors
    /// This method should halt if the given operand is not a literal.
    /// This method should halt if the register locator is not found.
    /// In the case of register members, this method should halt if the member is not found.
    #[inline]
    fn load_literal_circuit(
        &self,
        stack: &(impl StackMatches<N> + StackProgram<N>),
        operand: &Operand<N>,
    ) -> Result<circuit::Literal<A>> {
        match self.load_circuit(stack, operand)? {
            circuit::Value::Plaintext(circuit::Plaintext::Literal(literal, ..)) => Ok(literal),
            circuit::Value::Plaintext(circuit::Plaintext::Struct(..)) => bail!("Operand must be a literal"),
            circuit::Value::Record(..) => bail!("Operand must be a literal"),
        }
    }

    /// Loads the plaintext of a given operand.
    ///
    /// # Errors
    /// This method should halt if the given operand is not a plaintext.
    /// This method should halt if the register locator is not found.
    /// In the case of register members, this method should halt if the member is not found.
    #[inline]
    fn load_plaintext_circuit(
        &self,
        stack: &(impl StackMatches<N> + StackProgram<N>),
        operand: &Operand<N>,
    ) -> Result<circuit::Plaintext<A>> {
        match self.load_circuit(stack, operand)? {
            circuit::Value::Plaintext(plaintext) => Ok(plaintext),
            circuit::Value::Record(..) => bail!("Operand must be a plaintext"),
        }
    }
}

pub trait RegistersStore<N: Network> {
    /// Assigns the given value to the given register, assuming the register is not already assigned.
    ///
    /// # Errors
    /// This method should halt if the given register is a register member.
    /// This method should halt if the given register is an input register.
    /// This method should halt if the register is already used.
    fn store(
        &mut self,
        stack: &(impl StackMatches<N> + StackProgram<N>),
        register: &Register<N>,
        stack_value: Value<N>,
    ) -> Result<()>;

    /// Assigns the given literal to the given register, assuming the register is not already assigned.
    ///
    /// # Errors
    /// This method should halt if the given register is a register member.
    /// This method should halt if the given register is an input register.
    /// This method should halt if the register is already used.
    #[inline]
    fn store_literal(
        &mut self,
        stack: &(impl StackMatches<N> + StackProgram<N>),
        register: &Register<N>,
        literal: Literal<N>,
    ) -> Result<()> {
        self.store(stack, register, Value::Plaintext(Plaintext::from(literal)))
    }
}

pub trait RegistersStoreCircuit<N: Network, A: circuit::Aleo<Network = N>> {
    /// Assigns the given value to the given register, assuming the register is not already assigned.
    ///
    /// # Errors
    /// This method should halt if the given register is a register member.
    /// This method should halt if the given register is an input register.
    /// This method should halt if the register is already used.
    fn store_circuit(
        &mut self,
        stack: &(impl StackMatches<N> + StackProgram<N>),
        register: &Register<N>,
        stack_value: circuit::Value<A>,
    ) -> Result<()>;

    /// Assigns the given literal to the given register, assuming the register is not already assigned.
    ///
    /// # Errors
    /// This method should halt if the given register is a register member.
    /// This method should halt if the given register is an input register.
    /// This method should halt if the register is already used.
    #[inline]
    fn store_literal_circuit(
        &mut self,
        stack: &(impl StackMatches<N> + StackProgram<N>),
        register: &Register<N>,
        literal: circuit::Literal<A>,
    ) -> Result<()> {
        self.store_circuit(stack, register, circuit::Value::Plaintext(circuit::Plaintext::from(literal)))
    }
}

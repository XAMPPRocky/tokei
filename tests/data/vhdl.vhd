-- 30 lines 20 code 4 comments 6 blanks

library IEEE;
use IEEE.STD_LOGIC_1164.ALL;
use IEEE.NUMERIC_STD.ALL;

entity tb is
    Port ( clk : in STD_LOGIC; -- clock
           rst : in STD_LOGIC; -- reset
           -- removed: in STD_LOGIC_VECTOR(7 downto 0)
         );
end tb;

-- architecture
architecture behavioural of tb is
    signal toggle : STD_LOGIC := '0';

begin

    -- Toggles signal
    process(clk, rst)
    begin
        if (rst='1') then
            toggle <= '0';
        else
            toggle <= not toggle;
        end if;
    end process;

end

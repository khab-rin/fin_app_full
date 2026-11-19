select contract_id as "contract_id: BoxUuid"
from contracts
where comp_id = $1
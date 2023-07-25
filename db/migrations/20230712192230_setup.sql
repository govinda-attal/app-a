CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE AUCTION_STATUS AS ENUM(
	'DRAFT',
	'OPEN',
	'CLOSED',
	'CANCELLED'
);

CREATE TYPE BID_STATUS AS ENUM(
	'DRAFT',
	'ACCEPTED',
	'REJECTED',
	'OVER_TURNED'
);

CREATE TABLE AUCTIONS (
	ID uuid DEFAULT uuid_generate_v4 (),
	ITEM text NOT NULL,
	DESCRIPTION text NOT NULL,
	SELLER text NOT NULL,
	START_PRICE integer NOT NULL,
	CURRENT_PRICE integer,
	STATUS AUCTION_STATUS NOT NULL,
	CREATED_AT timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
	LAST_UPDATED_AT timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
	PRIMARY KEY (ID)
);


CREATE TABLE BIDS (
	ID uuid DEFAULT uuid_generate_v4 (),
	AUCTION_ID uuid NOT NULL,
	BIDDER text NOT NULL,
	BID_PRICE integer NOT NULL,
	STATUS BID_STATUS NOT NULL,
	CREATED_AT timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
	LAST_UPDATED_AT timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
	PRIMARY KEY (ID),
    FOREIGN KEY (AUCTION_ID) REFERENCES AUCTIONS (ID)
);

CREATE FUNCTION update_last_updated_auctions() RETURNS TRIGGER AS $$ BEGIN NEW.LAST_UPDATED_AT = now();
RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_auctions_last_updated BEFORE
UPDATE
	ON AUCTIONS FOR EACH ROW EXECUTE PROCEDURE update_last_updated_auctions();

CREATE FUNCTION update_last_updated_bids() RETURNS TRIGGER AS $$ BEGIN NEW.LAST_UPDATED_AT = now();
RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_bids_last_updated BEFORE
UPDATE
	ON BIDS FOR EACH ROW EXECUTE PROCEDURE update_last_updated_bids();


CREATE INDEX auctions_seller_index ON AUCTIONS (SELLER) WITH (deduplicate_items = off);
CREATE INDEX auctions_status_index ON AUCTIONS (STATUS) WITH (deduplicate_items = off);
CREATE INDEX auctions_last_updated_at_index ON AUCTIONS (LAST_UPDATED_AT DESC) WITH (deduplicate_items = off);

CREATE INDEX bids_bidder_index ON BIDS (BIDDER) WITH (deduplicate_items = off);
CREATE INDEX bids_bid_price_index ON BIDS (BID_PRICE DESC) WITH (deduplicate_items = off);
CREATE INDEX bids_status_index ON BIDS (STATUS) WITH (deduplicate_items = off);